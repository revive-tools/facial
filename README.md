# Facial recognition for Node.js

This library provides bindings to [`dlib-face-recognition`](https://crates.io/crates/dlib-face-recognition) for Node.js using [napi.rs](https://napi.rs)

- [Supported platforms](#supported-platforms)
- [Supported image formats](#supported-image-formats)
- [Usage](#usage)
- [Using with elasticsearch](#using-with-elasticsearch)

## Supported platforms

|                  | >=16.x |
| ---------------- | ------ |
| macOS x64        |  ✓     |
| macOS arm64      |  ✓     |
| Linux x64 gnu    |  ✓     |
| Linux x64 musl   |  ✓     |
| Linux arm gnu    |  ✓     |
| Linux arm64 gnu  |  ✓     |
| Linux arm64 musl |  ✓     |

## Supported image formats

| Format   | Supported                                 |
| -------- | ----------------------------------------- |
| PNG      | All supported color types                 |
| JPEG     | Baseline and progressive                  |
| GIF      | Yes                                       |
| BMP      | Yes                                       |
| ICO      | Yes                                       |
| TIFF     | Baseline(no fax support) + LZW + PackBits |
| WebP     | Yes                                       |
| AVIF     | Only 8-bit	                               |
| PNM      | PBM, PGM, PPM, standard PAM               |
| DDS      | DXT1, DXT3, DXT5                          |
| TGA      | Yes                                       |
| OpenEXR  | Rgb32F, Rgba32F (no dwa compression)      |
| farbfeld | Yes                                       |

_See [`image`](https://crates.io/crates/image) crate for more info_

## Installation
```console
yarn add @revive-tools/facial
```

```console
npm i -S @revive-tools/facial
```

## Usage

1. Create `Facial` instance from image.

```typescript
import { Facial } from '@revive-tools/facial'

const faces = await got('https://thispersondoesnotexist.com/image').buffer()

const facial = Facial.fromImage(faces)
```

2. Extract face locations

```typescript
let locations = facial.locations()
console.log(locations) // -> [..., { left: 461, top: 189, right: 533, bottom: 261 }, ...]
```

3. Extract face landmarks

```typescript
let landmarks = facial.landmarks(locations)
console.log(landmarks) // -> [..., [..., { x: 495, y: 242 }, { x: 502, y: 244 }, { x: 496, y: 246 }, ...] ...]
```

4. Extract a list of 128-dimensional face encodings

```typescript
let encodings = facial.encodings(location[0])
console.log(encodings) // -> [-0.09409232437610626, 0.059768397361040115, 0.04812365770339966, -0.011577781289815903, -0.07372716814279556,  -0.08940821886062622, ...]
```

5. Compare faces

```typescript
import { compareFaces } from '@revive-tools/facial'
let one = facial.encodings(location[0])
let two = facial.encodings(location[0])

// A float value from 0 to 1
const similarity = compareFaces(one, two)

console.log(`Faces are ${similarity * 100}% similar`)
```

## Using with elasticsearch
You can achieve horizontally scalability by storing and querying encodings in elasticsearch

[_see elasticsearch blog post_](https://www.elastic.co/blog/how-to-build-a-facial-recognition-system-using-elasticsearch-and-python)

1. Creating index

```bash
curl -XPUT "http://localhost:9200/faces" -H 'Content-Type: application/json' -d' 
{ 
  "mappings" : { 
      "properties" : { 
        "face_name" : { 
          "type" : "keyword" 
        }, 
        "face_encoding" : { 
          "type" : "dense_vector", 
          "dims" : 128 
        } 
      } 
    } 
}'
```

2. Add faces

```typescript
import { Client } from '@elastic/elasticsearch'

const client = new Client({
  node: 'http://localhost:9200',
})

export const addFace = async (encodings: number[], name: string) => {
  await client.create({
    document: {
      face_encoding: encodings,
      face_name: name,
    },
    id: name,
    index: 'faces',
  })
}
```

3. Query them

```typescript
export const searchFace = async (encodings: number[], size = 3) => {
  const body = {
    _source: 'face_name',
    query: {
      script_score: {
        query: {
          match_all: {},
        },
        script: {
          params: {
            query_vector: encodings,
          },
          source: "cosineSimilarity(params.query_vector, 'face_encoding')",
        },
      },
    },
    size,
  }

  const data = await raw
    .post('faces/_search', {
      body: JSON.stringify(body),
      headers: {
        'Content-Type': 'application/json',
      },
    })
    .json<SearchResponse>()

  return data.hits.hits.map((hit) => {
    return {
      name: (hit._source as any).face_name as string,
      score: hit._score as number,
    }
  })
}
```

_I'm using [`got`](https://www.npmjs.com/package/got) instead of elastic client in this example, because `got` is more performant, and elastic client library under pressure creates a memory leak._


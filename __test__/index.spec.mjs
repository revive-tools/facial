import { Facial, compareFaces } from '../index.js'

import got from 'got'
import test from 'ava'

let facial
let locations

test.before('create facial instance from image', async (t) => {
  const faces = await got('https://upload.wikimedia.org/wikipedia/commons/thumb/5/58/Michael_E._Arth_photograph_of_crowd_in_Jilin%2C_China%2C_1978.jpg/640px-Michael_E._Arth_photograph_of_crowd_in_Jilin%2C_China%2C_1978.jpg').buffer()
  facial = Facial.fromImage(faces)
})

test('detect multiple faces locations', async (t) => {
  locations = facial.locations()
  t.truthy(locations.length)
})

test('extract multiple face landmarks', async (t) => {
  const landmarks = facial.landmarks(locations)
  t.truthy(landmarks.length)
})

test('extract and compare face encodings', async (t) => {
  const [one, two] = locations.map((location) => {
    const encodings = facial.encodings(location, 1)
    t.is(encodings.length, 128)
    return encodings
  })

  t.true(compareFaces(one, two) < 0.5, 'different faces are not similar')
  t.false(compareFaces(one, one) > 0.5, 'same face is similar')
})

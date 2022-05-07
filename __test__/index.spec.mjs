import { faceEncodings, faceLandmarks, faceLocations } from '../index.js'

import got from 'got'
import test from 'ava'

const face = await got('https://thispersondoesnotexist.com/image').buffer()
const faces = await got('https://upload.wikimedia.org/wikipedia/commons/thumb/5/58/Michael_E._Arth_photograph_of_crowd_in_Jilin%2C_China%2C_1978.jpg/640px-Michael_E._Arth_photograph_of_crowd_in_Jilin%2C_China%2C_1978.jpg').buffer()

let location
let locations

test('detect one face location', async (t) => {
  location = faceLocations(face)[0]
  t.truthy(location)
})

test('detect multiple faces locations', async (t) => {
  locations = faceLocations(faces)
  t.truthy(locations.length)
})

test('extract one face landmarks', async (t) => {
  const landmarks = faceLandmarks(face, location)
  t.truthy(landmarks.length)
})

test('extract multiple face landmarks', async (t) => {
  return Promise.all(locations.map(async (location) => {
    const landmarks = faceLandmarks(face, location)
    t.truthy(landmarks.length)
  }))
})

test('extract one face encodings', async (t) => {
  const encodings = faceEncodings(face, location)
  t.is(encodings.length, 128)
})

test('extract multiple faces encodings', async (t) => {
  return Promise.all(locations.map(async (location) => {
    const encodings = faceEncodings(face, location)
    t.is(encodings.length, 128)
  }))
})

import { faceLocations } from '../index.js'
import got from 'got'
import test from 'ava'

const face = await got('https://thispersondoesnotexist.com/image').buffer()
const faces = await got('https://upload.wikimedia.org/wikipedia/commons/thumb/5/58/Michael_E._Arth_photograph_of_crowd_in_Jilin%2C_China%2C_1978.jpg/640px-Michael_E._Arth_photograph_of_crowd_in_Jilin%2C_China%2C_1978.jpg').buffer()

test('detect one face location', async (t) => {
  const [location] = faceLocations(face)
  t.truthy(location)
})

test('detect multiple faces locations', async (t) => {
  const locations = faceLocations(faces)
  t.truthy(locations.length)
})

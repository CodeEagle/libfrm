import test from 'ava'

import { projectsOf } from '../index'

test('test', (t) => {
  const dir = process.cwd()
  const list = projectsOf(dir)
  t.is(list.length, 0)
})

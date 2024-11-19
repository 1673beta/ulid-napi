import test from 'ava'

import { decodeTime, isValid, ulid } from '../index.js'

test('isValid', (t) => {
  t.is(isValid('01ARYZ6S41TSV4RRFFQ69G5FAV'), true)
  t.is(isValid('01ARYZ6S41TSV4RRFFQ69G5FA'), false)
})

test('decodeTime', (t) => {
  t.is(decodeTime('01JCZW9GR1BTE6Z1SNGZ40A4XH'), 1731941679873)
})

test('generate ULID', (t) => {
  t.is(ulid().length, 26)
})
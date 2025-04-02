import { Knex } from 'knex';

export interface PaymentEvent {
  id?: string;
  providerEventId: string;
  eventData: string;
  createdAt?: Date;
  updatedAt?: Date;
}

const fixResultsetTypes = (r: any) =>
  r.map((c: any) => ({
    ...c,
    modelArgs: JSON.stringify(c.eventData), // because we're using a jsonb column
  }));

const insert = (db: Knex) => (paymentEvent: PaymentEvent) =>
  db
    .insert({ ...paymentEvent, createdAt: new Date() })
    .into('paymentEvent')
    .returning('*')
    .then(fixResultsetTypes)
    .then((r) => r[0]);

const upsert = (db: Knex) => (paymentEvent: PaymentEvent) =>
  db
    .insert({ ...paymentEvent, createdAt: new Date() })
    .into('paymentEvent')
    .onConflict('id')
    .merge({ ...paymentEvent, updatedAt: new Date() })
    .returning('*')
    .then(fixResultsetTypes)
    .then((r) => r[0]);

const update = (db: Knex) => (paymentEvent: PaymentEvent) =>
  db
    .update({ ...paymentEvent, updatedAt: new Date() })
    .from('paymentEvent')
    .where('id', paymentEvent.id)
    .returning('*')
    .then(fixResultsetTypes)
    .then((r) => r[0]);

const get =
  (db: Knex) =>
    (id: string): Promise<PaymentEvent> =>
      db
        .select()
        .from('paymentEvent')
        .where('id', id)
        .then(fixResultsetTypes)
        .then((r) => (r.length ? r[0] : null));

const getAll = (db: Knex) => () =>
  db
    .select()
    .from('paymentEvent')
    .orderBy('createdAt', 'desc')
    .then(fixResultsetTypes);

const getAllByCustomerEmail = (db: Knex) => (customerEmail: string) =>
  db
    .select()
    .from('paymentEvent')
    .where('customerEmail', customerEmail)
    .orderBy('createdAt', 'desc')
    .then(fixResultsetTypes);

const getAllByUserId = (db: Knex) => (userId: string) =>
  db
    .select()
    .from('paymentEvent')
    .where('userId', userId)
    .orderBy('createdAt', 'desc')
    .then(fixResultsetTypes)

export default (db: Knex) => ({
  insert: insert(db),
  upsert: upsert(db),
  update: update(db),
  get: get(db),
  getAll: getAll(db),
  getAllByCustomerEmail: getAllByCustomerEmail(db),
  getAllByUserId: getAllByUserId(db),
});

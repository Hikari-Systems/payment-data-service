import { Knex } from 'knex';

export interface UserPaymentState {
  id?: string;
  userId: string;
  sku: string;
  providerProductId: string;
  providerPriceId: string;
  paidAt: Date;
  expiresAt: Date;
  refundedAt?: Date;
  customerId: string;
  plan: string;
  createdAt?: Date;
  updatedAt?: Date;
}

const insert = (db: Knex) => (userPaymentState: UserPaymentState) =>
  db
    .insert({
      ...userPaymentState,
      createdAt: new Date(),
    })
    .into('userPaymentState')
    .returning('*')
    .then((r) => r[0]);

const upsert = (db: Knex) => (userPaymentState: UserPaymentState) =>
  db
    .insert({
      ...userPaymentState,
      createdAt: new Date(),
    })
    .into('userPaymentState')
    .onConflict('id')
    .merge({
      ...userPaymentState,
      updatedAt: new Date(),
    })
    .returning('*')
    .then((r) => r[0]);

const update = (db: Knex) => (userPaymentState: UserPaymentState) =>
  db
    .update({
      ...userPaymentState,
      updatedAt: new Date(),
    })
    .from('userPaymentState')
    .where('id', userPaymentState.id)
    .returning('*')
    .then((r) => r[0]);

const get =
  (db: Knex) =>
  (id: string): Promise<UserPaymentState> =>
    db
      .select()
      .from('userPaymentState')
      .where('id', id)
      .then((r) => (r.length ? r[0] : null));

const getAll = (db: Knex) => () =>
  db.select().from('userPaymentState').orderBy('createdAt', 'desc');

const getByUserIdAndSku = (db: Knex) => (userId: string, sku: string) =>
  db
    .select()
    .from('userPaymentState')
    .where({ userId, sku })
    .orderBy('createdAt', 'desc');

const getAllByUserId = (db: Knex) => (userId: string) =>
  db
    .select()
    .from('userPaymentState')
    .where('userId', userId)
    .orderBy('createdAt', 'desc');

export default (db: Knex) => ({
  insert: insert(db),
  upsert: upsert(db),
  update: update(db),
  get: get(db),
  getAll: getAll(db),
  getByUserIdAndSku: getByUserIdAndSku(db),
  getAllByUserId: getAllByUserId(db),
});

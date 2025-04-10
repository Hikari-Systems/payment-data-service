import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema.table('userPaymentState', (t: Knex.TableBuilder) => {
    t.timestamp('refundedAt');
  });

export const down = (knex: Knex) =>
  knex.schema.table('userPaymentState', (t: Knex.TableBuilder) => {
    t.dropColumn('refundedAt');
  });

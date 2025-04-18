import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema.table('userPaymentState', (t: Knex.TableBuilder) => {
    t.string('customerId').notNullable().defaultTo('');
    t.string('plan').notNullable().defaultTo('');
  });

export const down = (knex: Knex) =>
  knex.schema.table('userPaymentState', (t: Knex.TableBuilder) => {
    t.dropColumn('customerId');
    t.dropColumn('plan');
  });

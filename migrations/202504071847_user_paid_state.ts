import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema
    .createTable('userPaymentState', (t: Knex.CreateTableBuilder) => {
      t.uuid('id').primary().notNullable();
      t.uuid('userId').notNullable();
      t.string('sku', 100).notNullable();
      t.timestamp('paidAt').notNullable();
      t.timestamp('expiresAt').notNullable();
      t.timestamps();
      t.index(['userId', 'sku']);
    });

export const down = (knex: Knex) =>
  knex.schema.dropTable('userPaymentState');

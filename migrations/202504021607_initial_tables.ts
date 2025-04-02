import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema
    .createTable('paymentEvent', (t: Knex.CreateTableBuilder) => {
      t.uuid('id').primary().notNullable();
      t.string('providerEventId', 100).notNullable();
      t.jsonb('eventData').notNullable();
      t.timestamps();
    });

export const down = (knex: Knex) =>
  knex.schema.dropTable('paymentEvent');

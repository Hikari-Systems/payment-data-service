import { Knex } from 'knex';

export const up = (knex: Knex) =>
  knex.schema
    .createTable('paymentEvent', (t: Knex.CreateTableBuilder) => {
      t.uuid('id').primary().notNullable();
      t.uuid('userId');
      t.string('customerEmail').notNullable();
      t.jsonb('eventData').notNullable();
      t.timestamps();
    });

export const down = (knex: Knex) =>
  knex.schema.dropTable('paymentEvent');

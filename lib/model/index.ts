import { knex, Knex } from 'knex';
import knexFile from '../knexfile';
import paymentEvent from './payment_event';
import userPaymentState from './user_payment_state';

const db: Knex = knex(knexFile.main);

export const healthcheck = () => db.select().from('knex_migrations').limit(1);

export const shutdown = () => db.destroy();

export const paymentEventModel = paymentEvent(db);
export const userPaymentStateModel = userPaymentState(db);
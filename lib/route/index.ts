import express from 'express';
import paymentEventRoutes from './payment_event';
import userPaymentStateRoutes from './user_payment_state';

const router = express.Router();
router.use('/paymentEvent', paymentEventRoutes);
router.use('/userPaymentState', userPaymentStateRoutes);

export default router;

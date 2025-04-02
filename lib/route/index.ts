import express from 'express';
import paymentEventRoutes from './payment_event';

const router = express.Router();
router.use('/paymentEvent', paymentEventRoutes);

export default router;

import express from 'express';
import { v4 } from 'uuid';
import { logging } from '@hikari-systems/hs.utils';
import dayjs from 'dayjs';
import { userPaymentStateModel } from '../model';

const log = logging('routes:userPaymentState');

const router = express.Router();

router.get('/:id', async (req, res, next) => {
  const id = req.params.id as string;
  if (!id) {
    return res.status(400).send(`No id provided`);
  }
  try {
    const userPaymentState = await userPaymentStateModel.get(id);
    if (!userPaymentState) {
      log.debug(`no userPaymentState found for id ${id}`);
      return res.status(204).end();
    }
    return res.status(200).json(userPaymentState);
  } catch (e) {
    log.error(`Error fetching userPaymentState for id ${id}`, e);
    return next(e);
  }
});

router.get('/byUserId/:userId', async (req, res, next) => {
  const userId = req.params.userId as string;
  if (!userId) {
    return res.status(400).send(`No userId provided`);
  }
  try {
    const userPaymentStates =
      await userPaymentStateModel.getAllByUserId(userId);
    return res.status(200).json(userPaymentStates);
  } catch (e) {
    log.error(`Error fetching userPaymentStates for userId ${userId}`, e);
    return next(e);
  }
});

router.get('/byUserIdAndSku/:userId/:sku', async (req, res, next) => {
  const { userId, sku } = req.params;
  if (!userId || !sku) {
    return res.status(400).send(`Missing userId or sku`);
  }
  try {
    const userPaymentStates = await userPaymentStateModel.getByUserIdAndSku(
      userId,
      sku,
    );
    return res.status(200).json(userPaymentStates);
  } catch (e) {
    log.error(
      `Error fetching userPaymentStates for userId ${userId} and sku ${sku}`,
      e,
    );
    return next(e);
  }
});

router.post('/', express.json(), async (req, res, next) => {
  const {
    userId,
    sku,
    providerProductId,
    providerPriceId,
    customerId,
    plan,
    paidAt,
    expiresAt,
    refundedAt,
  } = req.body as {
    userId: string;
    sku: string;
    providerProductId: string;
    providerPriceId: string;
    customerId: string;
    plan: string;
    paidAt: string;
    expiresAt: string;
    refundedAt?: string;
  };
  try {
    const userPaymentState = await userPaymentStateModel.insert({
      id: v4(),
      userId,
      sku,
      providerProductId,
      providerPriceId,
      customerId,
      plan,
      paidAt: dayjs(paidAt).toDate(),
      expiresAt: dayjs(expiresAt).toDate(),
      refundedAt:
        (refundedAt || '') !== '' ? dayjs(refundedAt).toDate() : undefined,
    });
    return res.status(201).json(userPaymentState);
  } catch (e) {
    log.error(
      `Error adding userPaymentState for ${JSON.stringify(req.body)}`,
      e,
    );
    return next(e);
  }
});

router.put('/:id', express.json(), async (req, res, next) => {
  const id = req.params.id as string;
  if (!id) {
    return res.status(400).send(`No id provided`);
  }
  const {
    userId,
    sku,
    providerProductId,
    providerPriceId,
    customerId,
    plan,
    paidAt,
    expiresAt,
    refundedAt,
  } = req.body as {
    userId: string;
    sku: string;
    providerProductId: string;
    providerPriceId: string;
    customerId: string;
    plan: string;
    paidAt: string;
    expiresAt: string;
    refundedAt?: string;
  };
  try {
    const userPaymentState = await userPaymentStateModel.update({
      id,
      userId,
      sku,
      providerProductId,
      providerPriceId,
      customerId,
      plan,
      paidAt: dayjs(paidAt).toDate(),
      expiresAt: dayjs(expiresAt).toDate(),
      refundedAt:
        (refundedAt || '') !== '' ? dayjs(refundedAt).toDate() : undefined,
    });
    return res.status(200).json(userPaymentState);
  } catch (e) {
    log.error(
      `Error updating userPaymentState for ${JSON.stringify(req.body)}`,
      e,
    );
    return next(e);
  }
});

export default router;

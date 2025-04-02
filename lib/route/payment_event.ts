import express from 'express';
import { v4 } from 'uuid';
import { logging } from '@hikari-systems/hs.utils';
import { paymentEventModel } from '../model';

const log = logging('routes:paymentEvent');

const router = express.Router();

router.get('/:id', async (req, res, next) => {
  const id = req.params.id as string;
  if (!id) {
    return res.status(400).send(`No id provided`);
  }
  try {
    const paymentEvent = await paymentEventModel.get(id);
    if (!paymentEvent) {
      log.debug(`no paymentEvent found for id ${id}`);
      return res.status(204).end();
    }
    return res.status(200).json(paymentEvent);
  } catch (e) {
    log.error(`Error fetching paymentEvent for id ${id}`, e);
    return next(e);
  }
});

router.get('/byUserId/:userId', async (req, res, next) => {
  const userId = req.params.userId as string;
  if (!userId) {
    return res.status(400).send(`No userId provided`);
  }
  try {
    const paymentEvents = await paymentEventModel.getAllByUserId(userId);
    if (!paymentEvents) {
      log.debug(`no paymentEvents found for userId ${userId}`);
      return res.status(204).end();
    }
    return res.status(200).json(paymentEvents);
  } catch (e) {
    log.error(`Error fetching paymentEvents for userId ${userId}`, e);
    return next(e);
  }
});

router.get('/byCustomerEmail/:customerEmail', async (req, res, next) => {
  const customerEmail = req.params.customerEmail as string;
  if (!customerEmail) {
    return res.status(400).send(`No customerEmail provided`);
  }
  try {
    const paymentEvents = await paymentEventModel.getAllByCustomerEmail(customerEmail);
    if (!paymentEvents) {
      log.debug(`no paymentEvents found for customerEmail ${customerEmail}`);
      return res.status(204).end();
    }
    return res.status(200).json(paymentEvents);
  } catch (e) {
    log.error(`Error fetching paymentEvents for customerEmail ${customerEmail}`, e);
    return next(e);
  }
});

router.post('/', express.json(), async (req, res, next) => {
  const { userId, customerEmail, eventData } = req.body as {
    userId?: string;
    customerEmail: string;
    eventData: string;
  };
  try {
    const paymentEvent = await paymentEventModel.insert({
      id: v4(),
      userId: userId || undefined,
      customerEmail,
      eventData,
    });
    return res.status(201).json(paymentEvent);
  } catch (e) {
    log.error(`Error adding provider for ${JSON.stringify(req.body)}`, e);
    return next(e);
  }
});

router.put('/:id', express.json(), async (req, res, next) => {
  const id = req.params.id as string;
  if (!id) {
    return res.status(400).send(`No id provided`);
  }
  const { userId, customerEmail, eventData } = req.body as {
    userId?: string;
    customerEmail: string;
    eventData: string;
  };
  try {
    const paymentEvent = await paymentEventModel.update({
      id,
      userId: userId || undefined,
      customerEmail,
      eventData,
    });
    return res.status(200).json(paymentEvent);
  } catch (e) {
    log.error(`Error updating paymentEvent for ${JSON.stringify(req.body)}`, e);
    return next(e);
  }
});

export default router;

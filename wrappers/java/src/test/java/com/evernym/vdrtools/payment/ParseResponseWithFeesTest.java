package com.evernym.vdrtools.payment;

import com.evernym.vdrtools.payments.Payments;
import com.evernym.vdrtools.payments.UnknownPaymentMethodException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class ParseResponseWithFeesTest extends PaymentIntegrationTest {

	@Test
	public void testParseResponseWithFeesWorksForUnknownPaymentMethod() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownPaymentMethodException.class));

		Payments.parseResponseWithFees(paymentMethod, emptyObject).get();
	}
}

package com.evernym.vdrtools.payment;

import com.evernym.vdrtools.payments.Payments;
import com.evernym.vdrtools.payments.UnknownPaymentMethodException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class ParseGetPaymentSourcesWithFromResponseTest extends PaymentIntegrationTest {

	@Test
	public void testParseGetPaymentSourcesWithFromResponseTestWorksForUnknownPaymentMethod() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownPaymentMethodException.class));

		Payments.parseGetPaymentSourcesWithFromResponse(paymentMethod, emptyObject).get();
	}
}

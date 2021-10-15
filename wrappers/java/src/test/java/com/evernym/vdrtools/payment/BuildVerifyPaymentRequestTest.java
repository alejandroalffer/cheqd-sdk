package com.evernym.vdrtools.payment;

import com.evernym.vdrtools.payments.Payments;
import com.evernym.vdrtools.payments.UnknownPaymentMethodException;
import com.evernym.vdrtools.IndyIntegrationTest;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class BuildVerifyPaymentRequestTest extends PaymentIntegrationTest {

	@Test
	public void testBuildVerifyPaymentRequestWorksForUnknownPaymentMethod() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownPaymentMethodException.class));

		Payments.buildVerifyPaymentRequest(wallet, IndyIntegrationTest.DID_TRUSTEE, receipt).get();
	}
}

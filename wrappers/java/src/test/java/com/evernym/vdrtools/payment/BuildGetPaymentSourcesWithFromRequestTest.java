package com.evernym.vdrtools.payment;

import com.evernym.vdrtools.payments.IncompatiblePaymentException;
import com.evernym.vdrtools.payments.Payments;
import com.evernym.vdrtools.payments.UnknownPaymentMethodException;
import com.evernym.vdrtools.IndyIntegrationTest;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class BuildGetPaymentSourcesWithFromRequestTest extends PaymentIntegrationTest {

	@Test
	public void testBuildGetPaymentSourcesWithFromRequestWorksForUnknownPaymentMethod() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownPaymentMethodException.class));

		Payments.buildGetPaymentSourcesWithFromRequest(wallet, IndyIntegrationTest.DID_TRUSTEE, paymentAddress, 1).get();
	}

	@Test
	public void testBuildGetPaymentSourcesWithFromRequestWorksForInvalidPaymentAddress() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(IncompatiblePaymentException.class));

		Payments.buildGetPaymentSourcesWithFromRequest(wallet, IndyIntegrationTest.DID_TRUSTEE, "pay:null1", 1).get();
	}
}

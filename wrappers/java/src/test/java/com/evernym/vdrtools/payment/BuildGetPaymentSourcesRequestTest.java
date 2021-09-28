package com.evernym.vdrtools.payment;

import com.evernym.vdrtools.payments.IncompatiblePaymentException;
import com.evernym.vdrtools.payments.Payments;
import com.evernym.vdrtools.payments.UnknownPaymentMethodException;
import com.evernym.vdrtools.IndyIntegrationTest;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class BuildGetPaymentSourcesRequestTest extends PaymentIntegrationTest {

	@Test
	public void testBuildGetPaymentSourcesRequestWorksForUnknownPaymentMethod() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownPaymentMethodException.class));

		Payments.buildGetPaymentSourcesRequest(wallet, IndyIntegrationTest.DID_TRUSTEE, paymentAddress).get();
	}

	@Test
	public void testBuildGetPaymentSourcesRequestWorksForInvalidPaymentAddress() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(IncompatiblePaymentException.class));

		Payments.buildGetPaymentSourcesRequest(wallet, IndyIntegrationTest.DID_TRUSTEE, "pay:null1").get();
	}
}

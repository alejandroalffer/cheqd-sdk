package com.evernym.vdrtools.payment;

import com.evernym.vdrtools.InvalidStructureException;
import com.evernym.vdrtools.payments.Payments;
import com.evernym.vdrtools.payments.UnknownPaymentMethodException;
import com.evernym.vdrtools.IndyIntegrationTest;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class BuildSetTxnFeesRequestTest extends PaymentIntegrationTest {

	@Test
	public void testBuildSetTxnFeesRequestWorksForUnknownPaymentMethod() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownPaymentMethodException.class));

		Payments.buildSetTxnFeesRequest(wallet, IndyIntegrationTest.DID_TRUSTEE, paymentMethod, fees).get();
	}

	@Test
	public void testBuildSetTxnFeesRequestWorksForInvalidFees() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Payments.buildSetTxnFeesRequest(wallet, IndyIntegrationTest.DID_TRUSTEE, paymentMethod, "[txnType1:1, txnType2:2]").get();
	}
}

package com.evernym.vdrtools.payment;

import com.evernym.vdrtools.InvalidStructureException;
import com.evernym.vdrtools.payments.IncompatiblePaymentException;
import com.evernym.vdrtools.payments.Payments;
import com.evernym.vdrtools.payments.UnknownPaymentMethodException;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class BuildPaymentRequestTest extends PaymentIntegrationTest {

	@Test
	public void testBuildPaymentRequestWorksForUnknownPaymentMethod() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownPaymentMethodException.class));

		Payments.buildPaymentRequest(wallet, DID_TRUSTEE, inputs, outputs, null).get();
	}

	@Test
	public void testBuildPaymentRequestWorksForEmptyInputs() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Payments.buildPaymentRequest(wallet, DID_TRUSTEE, emptyArray, outputs, null).get();
	}

	@Test
	public void testBuildPaymentRequestWorksForIncompatiblePaymentMethods() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(IncompatiblePaymentException.class));

		Payments.buildPaymentRequest(wallet, DID_TRUSTEE, incompatibleInputs, outputs, null).get();
	}
}

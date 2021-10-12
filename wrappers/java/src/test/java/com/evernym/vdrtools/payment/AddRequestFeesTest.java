package com.evernym.vdrtools.payment;

import com.evernym.vdrtools.InvalidStructureException;
import com.evernym.vdrtools.payments.IncompatiblePaymentException;
import com.evernym.vdrtools.payments.Payments;
import com.evernym.vdrtools.payments.UnknownPaymentMethodException;
import com.evernym.vdrtools.IndyIntegrationTest;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class AddRequestFeesTest extends PaymentIntegrationTest {

	@Test
	public void testAddRequestFeesWorksForUnknownPaymentMethod() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownPaymentMethodException.class));

		Payments.addRequestFees(wallet, IndyIntegrationTest.DID_TRUSTEE, emptyObject, inputs, outputs, null).get();
	}

	@Test
	public void testAddRequestFeesWorksForEmptyInputs() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Payments.addRequestFees(wallet, IndyIntegrationTest.DID_TRUSTEE, emptyObject, emptyArray, outputs, null).get();
	}

	@Test
	public void testAddRequestFeesWorksForSeveralMethods() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(IncompatiblePaymentException.class));

		Payments.addRequestFees(wallet, IndyIntegrationTest.DID_TRUSTEE, emptyObject, incompatibleInputs, emptyObject, null).get();
	}
}

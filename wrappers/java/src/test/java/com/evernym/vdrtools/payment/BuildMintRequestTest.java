package com.evernym.vdrtools.payment;

import com.evernym.vdrtools.InvalidStructureException;
import com.evernym.vdrtools.payments.IncompatiblePaymentException;
import com.evernym.vdrtools.payments.Payments;
import com.evernym.vdrtools.payments.UnknownPaymentMethodException;
import com.evernym.vdrtools.IndyIntegrationTest;
import org.junit.Test;

import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;

public class BuildMintRequestTest extends PaymentIntegrationTest {

	@Test
	public void testBuildMintRequestWorksForUnknownPaymentMethod() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(UnknownPaymentMethodException.class));

		Payments.buildMintRequest(wallet, IndyIntegrationTest.DID_TRUSTEE, outputs, null).get();
	}

	@Test
	public void testBuildMintRequestWorksForEmptyOutputs() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		Payments.buildMintRequest(wallet, IndyIntegrationTest.DID_TRUSTEE, emptyArray, null).get();
	}

	@Test
	public void testBuildMintRequestWorksForIncompatiblePaymentMethods() throws Exception {
		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(IncompatiblePaymentException.class));

		Payments.buildMintRequest(wallet, IndyIntegrationTest.DID_TRUSTEE, incompatibleOutputs, null).get();
	}
}

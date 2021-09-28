package com.evernym.vdrtools.anoncreds;

import static org.hamcrest.CoreMatchers.*;

import com.evernym.vdrtools.InvalidStructureException;


import org.junit.Test;

import java.util.concurrent.ExecutionException;

public class IssuerCreateAndStoreCredentialDefinitionTest extends AnoncredsIntegrationTest {

	@Test
	public void testIssuerCreateAndStoreCredentialDefWorks() throws Exception {
	}

	@Test
	public void testIssuerCreateAndStoreCredentialDefWorksForInvalidSchemaJson() throws Exception {

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(InvalidStructureException.class));

		String schema = "{" +
				"           \"name\":\"name\"," +
				"           \"version\":1.0," +
				"           \"attr_names\":[\"name\"]" +
				"        }";

		Anoncreds.issuerCreateAndStoreCredentialDef(wallet, issuerDid, schema, "InvalidSchema", null, defaultCredentialDefinitionConfig).get();
	}
}

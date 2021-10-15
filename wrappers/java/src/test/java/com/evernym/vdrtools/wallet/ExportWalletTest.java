package com.evernym.vdrtools.wallet;

import com.evernym.vdrtools.IOException;
import com.evernym.vdrtools.IndyIntegrationTestWithSingleWallet;
import com.evernym.vdrtools.did.Did;
import org.junit.Test;

import java.io.File;
import java.util.concurrent.ExecutionException;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertTrue;


public class ExportWalletTest extends IndyIntegrationTestWithSingleWallet {

	@Test
	public void testExportWalletWorks() throws Exception {
		Did.createAndStoreMyDid(wallet, "{}").get();

		Wallet.exportWallet(wallet, EXPORT_CONFIG_JSON).get();

		assertTrue(new File(EXPORT_PATH).exists());
	}

	@Test
	public void testExportWalletWorksForExistsPath() throws Exception {
		assertTrue(new File(EXPORT_PATH).mkdir());

		thrown.expect(ExecutionException.class);
		thrown.expectCause(isA(IOException.class));

		Wallet.exportWallet(wallet, EXPORT_CONFIG_JSON).get();
	}
}
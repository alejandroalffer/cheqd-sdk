package com.evernym.sdk.vcx;

import com.evernym.sdk.vcx.credentialDef.CredentialDefApi;
import com.evernym.sdk.vcx.issuer.IssuerApi;
import com.evernym.sdk.vcx.proof.ProofApi;
import com.evernym.sdk.vcx.utils.UtilsApi;
import com.evernym.sdk.vcx.vcx.VcxApi;
import com.evernym.sdk.vcx.wallet.WalletApi;
import org.junit.jupiter.api.BeforeEach;
import org.junit.jupiter.api.DisplayName;
import org.junit.jupiter.api.Test;

import java.util.UUID;
import java.util.concurrent.ExecutionException;

public class IssuerApiTest {
    private String sourceId = "123";

    @BeforeEach
    void setup() throws Exception {
        System.setProperty(org.slf4j.impl.SimpleLogger.DEFAULT_LOG_LEVEL_KEY, "DEBUG");
        if (!TestHelper.vcxInitialized) {
            UtilsApi.setPoolHandle(1);
            WalletApi.setWalletHandle(1);
            VcxApi.vcxInitMinimal(TestHelper.VCX_CONFIG_TEST_MODE);
            TestHelper.vcxInitialized = true;
        }
    }

    @Test
    @DisplayName("create a credential")
    void createCredential() throws VcxException, ExecutionException, InterruptedException {
        int credDefHandle = TestHelper._createCredentialDef();
        String credDefId = CredentialDefApi.credentialDefGetCredentialDefId(credDefHandle).get();
        CredentialDefApi.credentialDefRelease(credDefHandle);
        int newCredDefHandle = TestHelper._createCredentialDefWithId(credDefId);
        String credData = "{\"name\":\"joe\",\"age\":\"41\"}";
        int result = TestHelper.getResultFromFuture(IssuerApi.issuerCreateCredential(sourceId, newCredDefHandle, null, credData, "cred-name", "0"));
        assert (result != 0);
    }
}

package com.evernym.sdk.vcx.credential;

public class CredentialAcceptOfferResult {
    public CredentialAcceptOfferResult(int credentialHandle, String credentialSerialized) {
        this.credentialHandle = credentialHandle;
        this.credentialSerialized = credentialSerialized;
    }

    public int getCredentialHandle(){
        return credentialHandle;
    }

    public void setCredentialHandle(int credentialHandle){
        this.credentialHandle = credentialHandle;
    }

    private int credentialHandle;

    private String credentialSerialized;

    public String getCredentialSerialized(){
        return credentialSerialized;
    }

    public void setCredentialSerialized(String credentialSerialized){
        this.credentialSerialized = credentialSerialized;
    }
}

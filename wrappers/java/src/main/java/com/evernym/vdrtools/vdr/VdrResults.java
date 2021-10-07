package com.evernym.vdrtools.vdr;

import com.evernym.vdrtools.IndyJava;

public final class VdrResults {
    private VdrResults() {
    }

    public static class PreparedTxnResult extends IndyJava.Result {
        private String context;
        private String signatureSpec;
        private byte[] bytesToSign;
        private String endorsementSpec;

        public PreparedTxnResult(String context, String signatureSpec, byte[] bytesToSign, String endorsementSpec) {
            this.context = context;
            this.signatureSpec = signatureSpec;
            this.bytesToSign = bytesToSign;
            this.endorsementSpec = endorsementSpec;
        }

        public String getContext() {
            return context;
        }

        public String getSignatureSpec() {
            return signatureSpec;
        }

        public byte[] getBytesToSign() {
            return bytesToSign;
        }

        public String getEndorsementSpec() {
            return endorsementSpec;
        }
    }

}

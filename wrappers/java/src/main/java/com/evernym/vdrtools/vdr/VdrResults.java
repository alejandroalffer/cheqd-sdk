package com.evernym.vdrtools.vdr;

import com.evernym.vdrtools.IndyJava;

public final class VdrResults {
    private VdrResults() {
    }

    public static class PreparedTxnResult extends IndyJava.Result {
        private String namespace;
        private String signatureSpec;
        private byte[] txnBytes;
        private byte[] bytesToSign;
        private String endorsementSpec;

        public PreparedTxnResult(String namespace, String signatureSpec, byte[] txnBytes, byte[] bytesToSign, String endorsementSpec) {
            this.namespace = namespace;
            this.signatureSpec = signatureSpec;
            this.txnBytes = txnBytes;
            this.bytesToSign = bytesToSign;
            this.endorsementSpec = endorsementSpec;
        }

        public String getNamespace() {
            return namespace;
        }

        public String getSignatureSpec() {
            return signatureSpec;
        }

        public byte[] getTxnBytes() {
            return txnBytes;
        }

        public byte[] getBytesToSign() {
            return bytesToSign;
        }

        public String getEndorsementSpec() {
            return endorsementSpec;
        }
    }

}

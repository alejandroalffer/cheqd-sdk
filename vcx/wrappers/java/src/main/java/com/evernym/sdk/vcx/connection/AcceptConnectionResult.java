package com.evernym.sdk.vcx.connection;

public class AcceptConnectionResult {
	public AcceptConnectionResult(
			int connectionHandle,
			String connectionSerialized) {
		this.connectionHandle = connectionHandle;
		this.connectionSerialized = connectionSerialized;
	}

	public int getConnectionHandle() {
		return connectionHandle;
	}

	public void setConnectionHandle(int connectionHandle) {
		this.connectionHandle = connectionHandle;
	}

	private int connectionHandle;

	private String connectionSerialized;

	public String getConnectionSerialized() {
		return connectionSerialized;
	}

	public void setConnectionSerialized(String connectionSerialized) {
		this.connectionSerialized = connectionSerialized;
	}
}

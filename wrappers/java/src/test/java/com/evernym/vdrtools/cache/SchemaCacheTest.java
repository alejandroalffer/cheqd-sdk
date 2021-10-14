package com.evernym.vdrtools.cache;

import com.evernym.vdrtools.utils.PoolUtils;
import org.junit.*;

import static org.junit.Assert.assertTrue;

public class SchemaCacheTest extends CacheIntegrationTest {

	@Test(timeout = PoolUtils.TEST_TIMEOUT_FOR_REQUEST_ENSURE)
	public void testGetSchemaWorks() throws Exception {
		postEntities();

		String defaultOptions = "{\"noCache\": false, \"noUpdate\": false, \"noStore\": false, \"minFresh\": -1}";

		String schema = Cache.getSchema(pool, wallet, DID, String.valueOf(schemaId), defaultOptions).get();
	}

	@Test
	public void testPurgeSchemaCacheWorks() throws Exception {
	    String defaultOptions = "{\"maxAge\": -1}";
		Cache.purgeSchemaCache(wallet, defaultOptions).get();
	}
}

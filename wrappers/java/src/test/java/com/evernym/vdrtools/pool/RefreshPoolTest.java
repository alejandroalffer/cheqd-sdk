package com.evernym.vdrtools.pool;

import com.evernym.vdrtools.IndyIntegrationTest;
import com.evernym.vdrtools.utils.PoolUtils;
import org.junit.Test;

import static org.junit.Assert.assertNotNull;

public class RefreshPoolTest extends IndyIntegrationTest {

	@Test
	public void testRefreshPoolWorks() throws Exception {
		Pool pool = PoolUtils.createAndOpenPoolLedger();
		assertNotNull(pool);
		openedPools.add(pool);

		pool.refreshPoolLedger().get();
	}
}

package com.evernym.vdrtools.pool;

import com.evernym.vdrtools.IndyIntegrationTest;
import com.evernym.vdrtools.utils.PoolUtils;
import org.junit.Test;

import static org.junit.Assert.assertNotNull;

public class ClosePoolTest extends IndyIntegrationTest {

	@Test
	public void testClosePoolWorks() throws Exception {
		Pool pool = PoolUtils.createAndOpenPoolLedger();
		assertNotNull(pool);
		openedPools.add(pool);

		pool.closePoolLedger().get();
		openedPools.remove(pool);
	}


	@Test
	public void testAutoCloseWorks() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();
		try (Pool pool = Pool.openPoolLedger(poolName, null).get()) {
			assertNotNull(pool);
		}
		Pool pool = Pool.openPoolLedger(poolName, null).get();
		openedPools.add(pool);
	}
}

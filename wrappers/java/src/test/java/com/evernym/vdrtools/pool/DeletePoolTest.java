package com.evernym.vdrtools.pool;

import com.evernym.vdrtools.IndyIntegrationTest;
import com.evernym.vdrtools.InvalidStateException;
import com.evernym.vdrtools.utils.PoolUtils;
import org.junit.Test;

import static org.hamcrest.CoreMatchers.isA;
import static org.junit.Assert.assertNotNull;

public class DeletePoolTest extends IndyIntegrationTest {

	@Test
	public void testDeletePoolWorks() throws Exception {
		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool.deletePoolLedgerConfig(poolName).get();
	}

	@Test
	public void testDeletePoolWorksForOpened() throws Exception {
		thrown.expectCause(isA(InvalidStateException.class));

		String poolName = PoolUtils.createPoolLedgerConfig();
		Pool pool = Pool.openPoolLedger(poolName, null).get();
		assertNotNull(pool);
		openedPools.add(pool);
		Pool.deletePoolLedgerConfig(poolName).get();
	}
}

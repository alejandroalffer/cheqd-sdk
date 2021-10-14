package com.evernym.vdrtools.pool;

import com.evernym.vdrtools.IndyIntegrationTest;
import com.evernym.vdrtools.utils.PoolUtils;

import org.junit.Test;
import org.json.JSONArray;

import java.io.File;

import static org.junit.Assert.assertTrue;
import static org.junit.Assert.assertEquals;

public class ListPoolsTest extends IndyIntegrationTest {

    @Test
    public void testListPoolsWorks() throws Exception {
        File file = new File("testListPoolsWorks.txn");
        file.deleteOnExit();
        assertTrue(file.createNewFile());
        PoolUtils.writeTransactions(file);

        String testPoolName = "testListPoolsWorks";

        Pool.createPoolLedgerConfig(testPoolName, null).get();
        String listPoolsJson = Pool.listPools().get();

        JSONArray listPools = new JSONArray(listPoolsJson);

        assertEquals(1, listPools.length());
        assertEquals(testPoolName, listPools.getJSONObject(0).getString("pool"));
    }

}

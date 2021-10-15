package com.evernym.vdrtools.utils;


import com.evernym.vdrtools.LibIndy;

public class InitHelper {
	public static void init() {

		if (!LibIndy.isInitialized()) LibIndy.init("./lib/");

	}
}

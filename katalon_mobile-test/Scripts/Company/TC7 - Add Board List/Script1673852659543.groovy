import static com.kms.katalon.core.checkpoint.CheckpointFactory.findCheckpoint
import static com.kms.katalon.core.testcase.TestCaseFactory.findTestCase
import static com.kms.katalon.core.testdata.TestDataFactory.findTestData
import static com.kms.katalon.core.testobject.ObjectRepository.findTestObject
import static com.kms.katalon.core.testobject.ObjectRepository.findWindowsObject
import com.kms.katalon.core.checkpoint.Checkpoint as Checkpoint
import com.kms.katalon.core.cucumber.keyword.CucumberBuiltinKeywords as CucumberKW
import com.kms.katalon.core.mobile.keyword.MobileBuiltInKeywords as Mobile
import com.kms.katalon.core.model.FailureHandling as FailureHandling
import com.kms.katalon.core.testcase.TestCase as TestCase
import com.kms.katalon.core.testdata.TestData as TestData
import com.kms.katalon.core.testng.keyword.TestNGBuiltinKeywords as TestNGKW
import com.kms.katalon.core.testobject.TestObject as TestObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webui.keyword.WebUiBuiltInKeywords as WebUI
import com.kms.katalon.core.windows.keyword.WindowsBuiltinKeywords as Windows
import internal.GlobalVariable as GlobalVariable
import org.openqa.selenium.Keys as Keys
import com.kms.katalon.core.configuration.RunConfiguration as RunConfiguration
import com.github.javafaker.Faker as Faker

Faker faker = new Faker();

Random random = new Random()

Mobile.tap(findTestObject('Menu Board/android.widget.Button add new list'), GlobalVariable.timeout)

Mobile.tap(findTestObject('Menu Board/EditText - Add new list'), GlobalVariable.timeout)

//String listboardName = 'Board List Automation'

String listboardName = "Board List Automation" + faker.name()

CustomKeywords.'helper.globalVariableUpdater.updatePermanently'(RunConfiguration.getExecutionProfile(), 'listboardName', 
    listboardName)

Mobile.sendKeys(findTestObject('Menu Board/EditText - Add new list'), listboardName)

Mobile.tap(findTestObject('Menu Board/Button submit new list'), GlobalVariable.timeout)


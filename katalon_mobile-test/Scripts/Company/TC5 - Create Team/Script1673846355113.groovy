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

Random random = new Random()

Mobile.tap(findTestObject('team/Button-Add'), GlobalVariable.timeout)

Mobile.tap(findTestObject('team/android.view.Add new team'), GlobalVariable.timeout)

Mobile.tap(findTestObject('team/android.widget.EditText - name team'), GlobalVariable.timeout)

String teamName = 'automation team'

CustomKeywords.'helper.globalVariableUpdater.updatePermanently'(RunConfiguration.getExecutionProfile(), 'teamName', teamName)

Mobile.sendKeys(findTestObject('team/android.widget.EditText - name team'), teamName)

Mobile.tap(findTestObject('team/android.widget.EditText - description team'), GlobalVariable.timeout)

String teamDesc = 'Description team'

CustomKeywords.'helper.globalVariableUpdater.updatePermanently'(RunConfiguration.getExecutionProfile(), 'teamDesc', teamDesc)

Mobile.sendKeys(findTestObject('team/android.widget.EditText - description team'), teamDesc)

Mobile.tap(findTestObject('team/android.widget.Button Create Team'), GlobalVariable.timeout)


<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create team 2.0</name>
   <tag></tag>
   <elementGuidId>83a0902c-fbf4-447c-b8ae-894452024633</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\t\n \&quot;data\&quot;:{\n    \&quot;name\&quot;: \&quot;${GlobaVariable.teamName}\&quot;,\n    \&quot;desc\&quot;: \&quot;team desc\&quot;,\n    \&quot;type\&quot;: \&quot;team\&quot;\n\t},\n \&quot;selector\&quot;:{\n\t\&quot;companyId\&quot;: \&quot;${GlobalVariable.companyId}\&quot;\n\t}\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${authorization}</value>
      <webElementGuid>a9cf6150-c234-4bd1-8211-251c48490a29</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>00cfc7b6-ed60-4921-a787-e1e14d571fc1</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>7.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseURL}/teams?companyId=${companyId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseURL</defaultValue>
      <description></description>
      <id>2d33a87e-b038-46f1-823a-d561e5262969</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.companyId</defaultValue>
      <description></description>
      <id>dac6ba6b-88dd-46cb-a87a-537be3ba805a</id>
      <masked>false</masked>
      <name>companyId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.authorization</defaultValue>
      <description></description>
      <id>7e0fcd98-024a-4f00-a8da-cf95223179ff</id>
      <masked>false</masked>
      <name>authorization</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.teamName</defaultValue>
      <description></description>
      <id>447e9cd6-c83e-4d11-bf66-dee9c81f173b</id>
      <masked>false</masked>
      <name>teamName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.teamId</defaultValue>
      <description></description>
      <id>bd93505d-3f34-4521-b075-90ba6acf9350</id>
      <masked>false</masked>
      <name>teamId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.boardId</defaultValue>
      <description></description>
      <id>31caba7e-066c-497a-9560-0b035fea340a</id>
      <masked>false</masked>
      <name>boardId</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>

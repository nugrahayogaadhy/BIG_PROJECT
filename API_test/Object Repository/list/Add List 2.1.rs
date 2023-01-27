<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Add List 2.1</name>
   <tag></tag>
   <elementGuidId>30c314fa-5c70-418f-a861-ab702fd36d6e</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\&quot;selector\&quot;:{\&quot;boardId\&quot;:\&quot;${GlobalVariable.boardId}\&quot;},\&quot;data\&quot;:{\&quot;name\&quot;:\&quot;${GlobaVariable.listName}\&quot;}}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>${GlobalVariable.authorization}</value>
      <webElementGuid>37707379-7141-450d-a026-ee3b3dbe447b</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>fdfc7eb4-aa2b-4898-9685-0c4ab3a81f9a</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>7.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseURL}/lists/?companyId=${GlobalVariable.companyId}</restUrl>
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
      <id>f2fc4bf8-ce7c-470b-9446-7cdf5f9b80d1</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.authorization</defaultValue>
      <description></description>
      <id>4e9f1054-d7d6-44ba-8ac3-c69274371262</id>
      <masked>false</masked>
      <name>authorization</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.companyId</defaultValue>
      <description></description>
      <id>1178f3dc-80ae-4c63-a119-c721c534384c</id>
      <masked>false</masked>
      <name>companyId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.boardId</defaultValue>
      <description></description>
      <id>94fe7f52-5287-4e1c-8317-3f7e6ff580fc</id>
      <masked>false</masked>
      <name>boardId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.listName</defaultValue>
      <description></description>
      <id>f7a64b30-702c-416b-bd9a-d99c1dbd5782</id>
      <masked>false</masked>
      <name>listName</name>
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

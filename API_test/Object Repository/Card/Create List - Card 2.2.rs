<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create List - Card 2.2</name>
   <tag></tag>
   <elementGuidId>a007504f-59a1-483d-bd54-890b42e07ba8</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;selector\&quot;: {\n        \&quot;boardId\&quot;: \&quot;63bbef8abab253e23339ce70\&quot;,\n        \&quot;listId\&quot;: \&quot;63bbf0a7faf784b05649a7d8\&quot;\n    },\n    \&quot;data\&quot;: {\n        \&quot;name\&quot;: \&quot;Card Baru(List Postman 123)\&quot;,\n        \&quot;isPublic\&quot;: true\n    }\n}\n&quot;,
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
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
   </httpHeaderProperties>
   <katalonVersion>7.5.5</katalonVersion>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${baseURL}/cards/?companyId=${companyId}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.baseURL</defaultValue>
      <description></description>
      <id>4bbd2795-95e6-4a6f-9c40-7fb7b8bab7a2</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.authorization</defaultValue>
      <description></description>
      <id>b929dc8a-0d63-4a23-b05b-09d72cc378e8</id>
      <masked>false</masked>
      <name>authorization</name>
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

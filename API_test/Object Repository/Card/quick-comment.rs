<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>quick-comment</name>
   <tag></tag>
   <elementGuidId>e1169d0c-898d-485b-9019-084f916a41bb</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;content\&quot;: \&quot;\u003cp\u003ehai haai hai\u003c/p\u003e\&quot;,\n    \&quot;mentionedUsers\&quot;: [],\n    \&quot;type\&quot;: \&quot;card\&quot;\n}&quot;,
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
   <restUrl>${baseURL}/cards/63bc37a0bab253e2333a18ad/comments?companyId=${companyId}</restUrl>
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
      <id>9df2507c-a04a-4505-b379-dc8036f62644</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.authorization</defaultValue>
      <description></description>
      <id>69c794ef-d0e4-441c-9774-11b41846371a</id>
      <masked>false</masked>
      <name>authorization</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.companyId</defaultValue>
      <description></description>
      <id>514bb35a-1d31-4da8-9a98-dad76e002883</id>
      <masked>false</masked>
      <name>companyId</name>
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

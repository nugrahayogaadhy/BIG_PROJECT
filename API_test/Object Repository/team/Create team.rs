<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Create team</name>
   <tag></tag>
   <elementGuidId>ab501d4d-98ad-4cb2-98ae-0ad4c95018ed</elementGuidId>
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
      <webElementGuid>4c18e5cb-aafe-48ff-a9a9-199e2e167d6e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>95889737-55d3-403a-a962-5d69654f2b6f</webElementGuid>
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
      <id>e4ee1352-3aaa-4e7b-b4a4-45b1171cc05f</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.companyId</defaultValue>
      <description></description>
      <id>da2a2bcb-e770-4f9e-9691-e68ff084b72e</id>
      <masked>false</masked>
      <name>companyId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.authorization</defaultValue>
      <description></description>
      <id>a3dc1f3b-6a89-44fd-947d-e8de91d8ba3a</id>
      <masked>false</masked>
      <name>authorization</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.teamName</defaultValue>
      <description></description>
      <id>158d9018-3f2a-4ff4-b951-8cfddee67447</id>
      <masked>false</masked>
      <name>teamName</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.teamId</defaultValue>
      <description></description>
      <id>dad6241f-ee8a-48de-b236-0e77ee8d9cc8</id>
      <masked>false</masked>
      <name>teamId</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.boardId</defaultValue>
      <description></description>
      <id>3808f730-1170-4f23-9608-3fd1d8e58454</id>
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

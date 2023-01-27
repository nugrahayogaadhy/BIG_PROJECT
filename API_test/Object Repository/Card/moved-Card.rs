<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>moved-Card</name>
   <tag></tag>
   <elementGuidId>e75a97ca-84bd-464c-8fd2-79651a5a482d</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;data\&quot;: {\n        \&quot;draggableId\&quot;: \&quot;63bc3728faf784b05649fc64\&quot;,\n        \&quot;type\&quot;: \&quot;card\&quot;,\n        \&quot;source\&quot;: {\n            \&quot;index\&quot;: 1,\n            \&quot;droppableId\&quot;: \&quot;63bc35a1faf784b05649fba7\&quot;\n        },\n        \&quot;reason\&quot;: \&quot;DROP\&quot;,\n        \&quot;mode\&quot;: \&quot;FLUID\&quot;,\n        \&quot;destination\&quot;: {\n            \&quot;droppableId\&quot;: \&quot;63bbf0a7faf784b05649a7d8\&quot;,\n            \&quot;index\&quot;: 1\n        },\n        \&quot;combine\&quot;: null\n    },\n    \&quot;selector\&quot;: {\n        \&quot;boardId\&quot;: \&quot;63bbef8abab253e23339ce70\&quot;\n    }\n}&quot;,
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
   <restRequestMethod>PATCH</restRequestMethod>
   <restUrl>${baseURL}/cards/position?companyId=${companyId}</restUrl>
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
      <id>7ce219c7-481f-44a2-bfdd-9539b1062b1f</id>
      <masked>false</masked>
      <name>baseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.authorization</defaultValue>
      <description></description>
      <id>5705d4de-69ef-4b8f-aced-58007da64219</id>
      <masked>false</masked>
      <name>authorization</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.companyId</defaultValue>
      <description></description>
      <id>c1d75bd2-ec37-40a2-ac47-25919c67bee1</id>
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

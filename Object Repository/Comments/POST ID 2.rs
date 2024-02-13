<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST ID 2</name>
   <tag></tag>
   <elementGuidId>f8b3b8e4-5469-49a0-bc12-386ae786d6fe</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n \&quot;title\&quot; : \&quot;${nama}\&quot;,\n  \&quot;body\&quot; : \&quot;${email}\&quot;,\n  \&quot;userId\&quot; :\&quot;${body}\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>401ae2af-40ad-4097-9f61-cae760cd9593</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.3.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>${GlobalVariable.url}/comments</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'fariz'</defaultValue>
      <description></description>
      <id>a2bf2976-fb45-46ba-9129-1fcecaef355d</id>
      <masked>false</masked>
      <name>nama</name>
   </variables>
   <variables>
      <defaultValue>'fariz@user.com'</defaultValue>
      <description></description>
      <id>4e495098-d59f-4c23-80c2-07fa565230db</id>
      <masked>false</masked>
      <name>email</name>
   </variables>
   <variables>
      <defaultValue>'ini body'</defaultValue>
      <description></description>
      <id>9fa0d033-d920-42bf-b348-f091f75e752a</id>
      <masked>false</masked>
      <name>body</name>
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

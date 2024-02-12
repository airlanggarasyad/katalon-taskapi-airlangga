<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>POST Create User</name>
   <tag></tag>
   <elementGuidId>9849a5b2-8dd2-4738-83dc-e6f10aa3ad2a</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n  \&quot;name\&quot;: \&quot;Jalu Potter\&quot;,\n  \&quot;username\&quot;: \&quot;JalPot\&quot;,\n  \&quot;email\&quot;: \&quot;jalupotter@bri.co.id\&quot;,\n  \&quot;address\&quot;: {\n    \&quot;street\&quot;: \&quot;Kulas Light\&quot;,\n    \&quot;suite\&quot;: \&quot;Apt. 556\&quot;,\n    \&quot;city\&quot;: \&quot;Gwenborough\&quot;,\n    \&quot;zipcode\&quot;: \&quot;92998-3874\&quot;,\n    \&quot;geo\&quot;: {\n      \&quot;lat\&quot;: \&quot;-37.3159\&quot;,\n      \&quot;lng\&quot;: \&quot;81.1496\&quot;\n    }\n  },\n  \&quot;phone\&quot;: \&quot;1-770-736-8031 x56442\&quot;,\n  \&quot;website\&quot;: \&quot;hildegard.org\&quot;,\n  \&quot;company\&quot;: {\n    \&quot;name\&quot;: \&quot;Romaguera-Crona\&quot;,\n    \&quot;catchPhrase\&quot;: \&quot;Multi-layered client-server neural-net\&quot;,\n    \&quot;bs\&quot;: \&quot;harness real-time e-markets\&quot;\n  }\n}&quot;,
  &quot;contentType&quot;: &quot;application/json&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>20399a85-00b3-404c-ae53-28b4a61a3dbb</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.2.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://jsonplaceholder.typicode.com/users</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
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

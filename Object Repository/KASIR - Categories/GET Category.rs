<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET Category</name>
   <tag></tag>
   <elementGuidId>40458e0d-411e-4907-a73d-30779962b8b0</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjEwYzFkMDA3LWEwZTMtNDRjOC05YjFlLTVjOTczOWIwNDVjOCIsImNvbXBhbnlJZCI6IjhlZDMxMGU1LTdhMTgtNDk3Yi04M2FjLTU0NTk1ZWRkNGYyZiIsImlhdCI6MTcyNTgwMjQ5NX0.ApA2GEuQtLfuyLvTSo2C14xt_-LuLkfJcw5cwao03-4</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjEwYzFkMDA3LWEwZTMtNDRjOC05YjFlLTVjOTczOWIwNDVjOCIsImNvbXBhbnlJZCI6IjhlZDMxMGU1LTdhMTgtNDk3Yi04M2FjLTU0NTk1ZWRkNGYyZiIsImlhdCI6MTcyNTgwMjQ5NX0.ApA2GEuQtLfuyLvTSo2C14xt_-LuLkfJcw5cwao03-4</value>
      <webElementGuid>9024df25-0a4b-4bab-9d52-9223386da030</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseURL}/categories/c5f0345c-6259-47e2-9c79-bbe8397e1ca4</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>'c5f0345c-6259-47e2-9c79-bbe8397e1ca4'</defaultValue>
      <description></description>
      <id>1c85a16e-2089-4ece-b032-77c0a21c6b25</id>
      <masked>false</masked>
      <name>categoryId</name>
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

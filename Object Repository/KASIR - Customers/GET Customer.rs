<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET Customer</name>
   <tag></tag>
   <elementGuidId>7e4ee2c0-f17e-4720-ac94-d813292491c7</elementGuidId>
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
      <webElementGuid>be2ed8c8-fe46-4fab-912c-03fd80157e63</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.6.0</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${GlobalVariable.baseURL}/customers/eccd54c8-7374-40e4-96c5-ca03f92a05e3</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.customerId</defaultValue>
      <description></description>
      <id>dcc6464e-99cf-4720-9abf-93f3200e4bfb</id>
      <masked>false</masked>
      <name>customerId</name>
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

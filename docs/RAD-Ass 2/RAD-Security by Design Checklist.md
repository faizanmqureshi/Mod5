#### 												**Module 5 - Computer Systems (2021-22)**         						  <img src="C:\Users\SarmahDK\AppData\Roaming\Typora\typora-user-images\image-20210602115950778.png" alt="image-20210602115950778" style="zoom:60%;" />                                                                          

####																	**Project**                                   

##### 																**Security by Design Checklist**

#####															**(Requirement Analysis Phase 1)**


| Team ID: | Project Title: | Mentor(s): |
| -------- | -------------- | ---------- |
| Group 42 | T.W.I.P | Alexandru Matco & Marten Voorberg |

**					**
**Steps to be performed:**

- You should select a minimum of one security mechanism from each  of the security requirements from authentication and authorization ( auditing  is not included here).

- The auditing requirements should be considered as suggested  in the table according to your application. Other than the normal check on  protecting log files, backup files, etc, you should also think about the GDPR obligations, software licensing, etc.  in line with your application.

- The given security mechanisms are for your inspiration. You  can select other mechanisms also according to the requirement of your  application. For example: If you select "authentication" as one  of the security requirements, the mechanism can be logging/password  checking, biometric, OAuth, etc. The same is applicable for authorization and  auditing.

- Justify the reason to select a particular mechanism for the requirements in  the given column ‘C’.

- Write supplement requirement(s) in the form of a user story or  Abuse case for the application (refer to the example given on the table, column ‘D’).  (The supplement requirements should be according to the goals and  non-functional requirement (s) identified for your application.)

- Write the possible risks involved for the supplement requirements (refer to  the example given in the table, column ‘E’).

- Write the resources/mechanisms/tools to avoid/mitigate those risks for  security controls (refer to the example of the column heading "Appropriate  Security Control" (column ‘F’))

- This document must be reviewed with the team members and approved by your  mentor(s)/TAs.

- Put  checkmark in the last column for all verified items.

- This  document should be appended to the Software Requirement Specification (SRS)  document.
**					**
<br>

  **Follow  these 5 points for each of the Security Mechanisms and write them under  Appropriate Security Controls**:

  (i) Supplement security requirements to avoid risk. 
  
  (ii) Write the requirement of the resources to mitigate such risks. For example:  The type of Authentication software, security tokens, password management  software, etc.
  
  (iii) Devise a plan/method (tentative) to work on the identified risks.
  
  (iv)  Review the documentation within your team.
  
  (v)  Approve the document by your mentor.

  
  
  
  
  | Security  Policy                                           | Confidentiality, Integrity, and Availability                 |                                                              |                                                              |                                                              |                                                              |                                                              |
  | ---------------------------------------------------------- | ------------------------------------------------------------ | :----------------------------------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
  | **<span style="color:blue">Security  Requirements</span>** | **<span style="color:blue">Security  mechanisms (List down for your application)</span>** | **<span style="color:blue">Remarks  on why you considered these requirements? (in a brief)</span>** | **<span style="color:blue">Supplement  requirements for your application       (user story/Abuse case)</span>** | **<span style="color:blue">Risk  identification/Threat Assessment (at least one risk identification/abuse case)</span>** | **<span style="color:blue">Appropriate  Security Controls</span>** | **<span style="color:blue">Tick ✔if  you have applied the given security controls as suggested in the left  column </span>** |
  | **1.** **Authentication**                                  | 4 digit pin code and username.                         |  A 4 digit pin code is an ideal way to authenticate the user, provided that the user input will be collected through a 5 inch touchscreen display. | **User story**: As a user, I want to login to the system by inputting the correct 4 digit pin code. <br/><br/> **Abuse case**: As an attacker, I have access to valid usernames and password combinations of other users. | 1. The pin code is exposed via the display when the user types it in. <br/><br/> 2. An attacker tries random pin code combinations. | 1. Create an option for the 4 digit pin code to be reset if the combination is compromised. <br/><br/> 2. After a number of failed login attempts, the device locks up for a set amount of time.                                                           |
  | **2. Authorization**                                       | User-based access control policy.        |  A user is allowed to access his/her own account but not other users’ accounts.                                                     | A user is allowed to access his/her own account but not other users’ accounts.   |  Data from other users’ accounts is compromised.    | 1. Sanitize user input by using regular expressions as whitelists for any user input. <br/><br/>2. Use parameterized SQL queries.                       |                                                              |
  | **3. Audit**                                               | Logging of user activity and login events.                                    |In case of a system breach, the logged files can be investigated to help determine the source of the breach. | **Abuse case**: As an attacker, I want to breach the system without any trace. | Attacker deletes or manipulates the log files before the breach is detected. | Encryption of log files using SHA 256.                        |                                                              |

  

| Team members’ reviewed             | <span style="color:blue">(Ruter, yes), (Mila, Yes), (Daan, Yes), (Egor, Yes), (Novojit, Yes), (Faizan, Yes)</span> |
| ---------------------------------- | ------------------------------------------------------------ |
| **Mentors’ reviewed and verified** | <span style="color:blue">**(Mentor 1, yes), (Mentor 2, Yes)**</span> |


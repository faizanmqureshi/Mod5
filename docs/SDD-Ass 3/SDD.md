#### **Module 5 - Computer Systems (2021-22)**  <img src="C:\Users\SarmahDK\AppData\Roaming\Typora\typora-user-images\image-20210602172621460.png" alt="image-20210602172621460" style="zoom:80%;" />

#### **Project**

##### **Software Design Document**


| Team ID: | Project Title: | Mentor(s): |
| -------- | -------------- | ---------- |
| Group 42 | T.W.I.P | Alexandru Matco & Marten Voorberg |

**					**
**Instructions:**

i) You must explain all the given sections clearly and concisely.

ii) You must fill in the basic information about your projects such as Project Name, Team Members, Team ID, and Mentor(s). 

iii) Make sure to consider the checklist of the Design phase provided in the Security by Design document.

iv) The length of the document should be 4 to 8 pages.
**					**

1. **Introduction**

      Two-Wheeler Indicator Panel, T.W.I.P.,  is a smart bicycle dashboard that is equipped with multiple sensors, collecting and analyzing data,  to give the rider insights about his/her performance. Whether someone is a hobbying cyclist or a serious rider, it has something for everyone. 
T.W.I.P. displays speed, acceleration, distance travelled, and a myriad of other parameters based on the riders preference. It also allows multiple user profiles with user authentication to keep your data safe and secure.

2. **Functional/Nonfunctional Requirements**
     
      As the requirements were not accepted by the TA’s as they were incomplete and did not fully cover our system, we changed them.
      
      *Functional requirements:*
      - <span style="color:blue">*The system should display the current and average speeds.*</span>
      - <span style="color:blue">*The system should display current and previous pacing (time/km).*</span>
      - <span style="color:blue">*The system should keep track of the total distance.*</span>
      - <span style="color:blue">*The system should keep track of the trip distance of the cycling session.*</span>
      - <span style="color:blue">*The user should be able to log in using the touchscreen.*</span>
      - <span style="color:blue">*The system should automatically turn on/off lights depending on ambient light.*</span>
      - <span style="color:blue">*The system should be able to calculate the distance travelled.*</span>
      <br />

      *Nonfunctional requirements:*
      - <span style="color:blue">*The speed should be measured accurately within 1 km/h between 5 km/h and 25 km/h.*</span>
      - <span style="color:blue">*The representation of pacing should be comprehebsible at a glance while cycling.*</span>
      - <span style="color:blue">*Distances calculated should be accurate within 1%.*</span>
      - <span style="color:blue">*The system should not draw the users attention away from the road.*</span>
      - <span style="color:blue">*The user should be able to comprehend the data on the screen within 2 seconds of looking at the screen.*</span>
      - <span style="color:blue">*The lights shouldn’t randomly flash when the LDR is covered momentarily.*</span>
      - <span style="color:blue">*The LDR output should be consistent for 5 seconds before switching the state of the lights.*</span>
      - <span style="color:blue">*Users should be able to delete their personal data.*</span>
      - <span style="color:blue">*Complex system functions should lock while the bike is in motion to prevent distraction while travelling.*</span>
      - <span style="color:blue">*The system should be able to differentiate between users*</span> 

      <br />
   

3. **Architectural Design**

      *Class Diagram:*

      ![classDiagram](docs/Diagrams/databaseSchema.png)

      Since we do not work in OOP, we have not made a class diagram for our program, but we made one for our database. 

      *Sequence Diagram:*

      ![sequenceDiagram](docs/Diagrams/sequenceDiagram.png)

      This sequence diagram illustrates how the user interacts with the system through the touchscreen display. The log in and registration phase is inside a loop, indicating that until the user is logged in, no data (speed, distance travelled, pace) will be displayed. 
Once the user is logged in, he will be able to see the real-time data. The user will also get access to two other buttons: the LED button and the Ride History button. With these buttons, the user can change the LED status and look at ride history.     

      *Use Case Diagram:*

      ![useCaseDiagram](docs/Diagrams/useCaseDiagram.png)

      This diagram illustrates all the use cases of the system. There are 7 main use cases, 3 generalization use cases, 2 include, and 2 exclude use cases. The user of the bike computer is the only actor in this system.

      *Data Flow Diagram:*

      ![dataFlowDiagram](docs/Diagrams/dataFlowDiagram.png)

      This diagram illustrates how data flows among the different entities in our system. The Hall Effect sensor and the user are the external entities. Real-time ride data is collected from the sensor and processed by the Raspberry Pi. Another process records the data in the ride data database. When a user asks for ride history, it is pulled from this database and displayed to the user. Similarly, the user data record is updated whenever a new user registers. The authentication process also queries data from the user record database to validate log-in credentials. 

4. **Product User Interface**

      We made the following wireframe using the tool Balsamiq:

      ![Sign_in](docs/Wireframe/Sign_in.png)
      ![Register](docs/Wireframe/Register.png)
      ![Homepage](docs/Wireframe/Homepage.png)

      When the user looks at the screen, they will see the sign-in page. They can use their username and 4-digit pin code to log in by using the touchscreen. If the user does not have an account, they can sign-up for one. A username and pin code have to be given to make an account and to verify that the user uses the correct pin code they have in mind, a confirmation pin code is needed. <br />
When the user is logged in / registered, they will see the homepage. They will be welcomed by a personal message saying “Hi, [name]!”. On the homepage, the user can see the distance, speed and pace, and they can control the LEDs with a switch. They can also check their history, delete that history and when the user doesn’t want an account anymore, they can delete their account.

5. **Prevention/Mitigation Criteria (Security Controls)**
      
      We do not have a system that is connected to the internet. The system will run on a local system on the Raspberry Pi itself. The system will be locked by one user, and will only be able to run the Front-end application ideally. This would allow us to not think too much about the security risks that come with designing. <br />
For logging in, we will insert a username and pin code and in case the you forget your pincode or think it is compromised you can just change it using option forgot your pincode by answering simple questions or you can just delete the profile and create a new profile <br />
Since system is not connected to anything as wifi and bluetooth will be disabled therfore it cannot be accesed from outside.
6. **The cost involved (if any)**

      There is no cost of implementing the security.The user can just change the pincode or create new profile.  we do not need extra monetary costs for security, However we had yto implement the sytem for changing pincode, delete and create new user which took little effort . 

7. **Conclusion**

      In conclusion, the T.W.I.P needs to provide an alternative for current mobile cycling apps and bike computers by eliminating shortcomings of the aforementioned products and providing interesting new features. Whent he user is going to use T.W.I.P, they can sign in with an existing account or make one when they do not have one. On the homepage, they will see the distance travelled,  speed and acceleration, and they can control the LEDs with a switch. They can also check their history, delete that history, or when the user doesn’t want an account anymore, they can delete their account. All this information and data will be stored locally on the pi. The cost of implementing security is small as this is the only thing we need.   

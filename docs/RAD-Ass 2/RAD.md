#### 												**Module 5 - Computer Systems (2021-22)**         						  <img src="C:\Users\SarmahDK\AppData\Roaming\Typora\typora-user-images\image-20210602115950778.png" alt="image-20210602115950778" style="zoom:60%;" />                                                                          

#### 																		**Project**                                   

##### **Requirement Analysis Document**


| Team ID: | Project Title: | Mentor(s): |
| -------- | -------------- | ---------- |
| Group 42 | T.W.I.P | Alexandru Matco & Marten Voorberg |



**					**

1. **Introduction**

   There are several existing applications that you can select as a base for your project. In this section, you need to give a small background of already existing applications. The following points are introduced to get to know the purpose of your application, limitations of the existing system on which your project is based, etc.  
     <br>

   **A.** **Purpose**: 

    You should know the purpose of creating your application. Write the reason for selecting this project by mentioning the usefulness, quality, etc. of the system.

      <span style="color:blue"> *The project, Two Wheeler Indicator Panel (T.W.I.P), is selected for the following reasons:*</span>

    - <span style="color:blue"> *The device will display information about a users cycling behaviour such as speed, swirling and distance travelled to make cycling more interesting to the user.*</span>

    - <span style="color:blue"> *Using this data, the device can create interesting facts, tips or games with the collected data, for example, display distance travelled as X times travelled to Paris.*</span>

    - <span style="color:blue"> *Currently, there are numerous bicycle computers already on the market, but they all have disadvantages, this project aims to create a device that adds solutions to those disadvantages.*</span>

    <br>

    **B. Limitations of the current system (If any):**

    List down the limitations of the currently existing similar systems. 

    <span style="color:blue">*The current limitations of already existing bicycle displays are:* </span>

    - <span style="color:blue">*Current bicycle displays have a high price range.*</span>
    - <span style="color:blue">*Affordable bike computers have a limited range of data that they can collect or display.*</span>
    - <span style="color:blue">*Mobile apps as bike computers require the user to use their phone and drain their batteries at a significant rate.*</span>
    - <span style="color:blue">*Due to the limited size, bicycle computers have a limited input selection.*</span>
    - <span style="color:blue">*The buttons on bike computers have different functions depending on how long or often you press them; they have a confusing UI.* </span>
    - <span style="color:blue">*Bike computers have limited connectivity to share data with user.*</span>
    <br>


    **C.  Intended Audience:**

    Write about the targeted audience who can have access to your product or the documents. **For example**  <span style="color:blue">*users/stakeholders (Mentor, Project Coordinator, Module Coordinator, Project Members, Any specific users, etc.)* </span>

    *​Our product will be targeting cycling enthusiasts including professional riders as well as the hobbyist. As it will provide them modern features all just in one small dashboard panel where they will be able view their bicycle speed, distance travelled, average trip speed, heart beat and the respiration levels. 
Beginners can also profit from the system, as it can see what the user needs to improve on.*


    **E.** **Define SMART Goals**: 

    This section is used to list down the target/expected results from the project. All the goals should be written in a SMART (Specific + Measurable + Attainable + Relevant + Time-bound) way.

    <span style="color:blue">*The goals for the project T.W.I.P. are as follows:*</span>


    |<span style="color:green">**Specific (What)**</span>|<span style="color:green">**Measurable (Up to)**</span>|<span style="color:green">**Attainable (How)**</span>|<span style="color:green">**Relevant (Why)**</span>|<span style="color:green">Time-bound (when)</span>|
    | :-: | :- | :- | :- | :- |
    |<span style="color:blue">*To let the user know about the bikes acceleration and speed.*</span> |<span style="color:blue">*By measuring and displaying the speed, acceleration in the display attached to the bike.*</span>|<span style="color:blue">*By using accelerometer, wheel speed sensor and then displaying it at the digital display.*</span>|<span style="color:blue">*To help users know how fast they are going and alert at the speed limit.*</span>|<span style="color:blue">*To finish the task during sprint 3.*</span>|
    |<span style="color:blue">*To let the user know about total distance.*</span>|<span style="color:blue">*By measuring and displaying the distance in distance bar of display.*</span>|<span style="color:blue">*By using acceleration, speed and total time, distance can be calculated and displayed on the digital display.*</span>|<span style="color:blue">*To help users know how much they have travelled.*</span>|<span style="color:blue">*To finish the task during sprint 3*</span>|
    |<span style="color:blue">*To let the user analyse his past rides.*</span> |<span style="color:blue">*By allowing the user to see their progress and stats over the past time.*</span>|<span style="color:blue">*By storing the data in the system storage and retrieving it.*</span>|<span style="color:blue">*To allow the user to gain insights from past rides.*</span>|<span style="color:blue">*To finish the task during sprint 3.*</span>|




    **F.** **Scope**:
    
    This section is required to write about the important resources to achieve the goals of your system. The technology 				used to develop your project (methods/algorithms, software requirements, hardware requirements), the duration of the 				project, and the project constraints should be included here. The project constraints can be any technical hiccups, lack of 				resources, internal and external conditions (boundary conditions), etc. that can help further to avoid the related 				problems in the future during execution. In short, you can utilize this section to write about the limitations and       				boundaries of your project. 

    *Our project will work using different algorithms, hardwares and softwares. It will be mostly programmed on raspberry pi. Following software and hardwares would be used to complete the project successfully:*

    - *<span style="color:green">*i) System boundaries (software and hardware):*</span>* 
      - <span style="color:blue">*Software:*</span>
        - <span style="color:blue">*Programming language Back-end : Rust* </span>
        - <span style="color:blue">*Programming language Front-end : HTML & CSS* </span>
      - <span style="color:blue">*Hardware:*</span>
        - <span style="color:blue">*Raspberry Pi 4* </span>
        - <span style="color:blue">*Power Supply (Battery)* </span>
        - <span style="color:blue">*SSD* </span>
        - <span style="color:blue">*5 Inch Touch screen display* </span>
        - <span style="color:blue">*Speedometer (Hall effect sensor)* </span>
        - <span style="color:blue">*LDR sensor* </span>

    - ​	<span style="color:green">**ii)* *Limitations:** </span>
      - <span style="color:blue">*Battery pack does not last very long.* </span>
      - <span style="color:blue">*Project is not waterproof.*</span>
      - <span style="color:blue">*Display is low brightness.*</span>
      - <span style="color:blue">*Screen could break if bike goes on extreme rough track.*</span>
      - <span style="color:blue">*UI might distract rider if used while riding.*</span>
<br>


2. **Product features** 

   This section describes the functionality that you want to have in your product such as the components used for the application and its functionality, appearance, performance in terms of speed/time, etc. You can specify them in the form of functional and non-functional requirements. <span style="color:blue">A minimum number of 7 requirements (9 in case of selecting an existing application) is to be expected for your application. That includes functional as well as non-functional requirements cumulatively. However, it is highly probable that you will need more than the minimum amount to fully cover all the requirements. </span>

   

   ​	**A.** **Functional requirements:** 

   ​		Write the requirements that are directly connected with the functionality of the application. 

  

     - <span style="color:blue">*The system should display the current and average speeds.*</span>
     - <span style="color:blue">*The system should display current and previous pacing (time/km).*</span>
     - <span style="color:blue">*The system should keep track of the total distance.*</span>
     - <span style="color:blue">*The system should keep track of the trip distance of the cycling session.*</span>
     - <span style="color:blue">*The user should be able to log in using the touchscreen.*</span>
     - <span style="color:blue">*The system should automatically turn on/off lights depending on ambient light.*</span>
     - <span style="color:blue">*The system should be able to calculate the distance travelled.*</span>


    <br>

   **B.** **Nonfunctional requirements:** 

   ​  Write the requirements that are not the specific actions for your application but improve the quality of the system. This can be related to the storage capacity, performance requirements,  Security requirements (Refer to the checklist given in SBD document-Phase 1), etc. 

   

     - <span style="color:blue">*The speed should be measured accurately within 1 km/h between 5 km/h and 25 km/h.*</span>
     - <span style="color:blue">*The representation of pacing should be comprehensible at a glance while cycling.*</span>
     - <span style="color:blue">*Distances calculated should be accurate within 1%.*</span>
     - <span style="color:blue">*The system should not draw the users attention away from the road.*</span>
     - <span style="color:blue">*The user should be able to comprehend the data on the screen within 2 seconds of looking at the screen.*</span>
     - <span style="color:blue">*The lights shouldn’t randomly flash when the LDR is covered momentarily.*</span>
     - <span style="color:blue">*The LDR output should be consistent for 5 seconds before switching the state of the lights.*</span>
     - <span style="color:blue">*Users should be able to delete their personal data.*</span>
     - <span style="color:blue">*Complex system functions should lock while the bike is in motion to prevent distraction while travelling.*</span>
     - <span style="color:blue">*The system should be able to differentiate between users.*</span> 


<br>

3. **Conclusion:** Write the concluding remarks here. You can do this by **highlighting noteworthy decisions and challenges** for the next phase that you recognized.



    <span style="color:blue">*In conclusion the T.W.I.P. needs to provide an alternative for current mobile cycling apps and bike computers by eleminating shortcomings of the aforementioned products and providing interesting new features. During the next phases of the design process decisions will be made regarding the exact information displayed to the user and how much the user will be able to influence the data or personalize the display output. Future challenges will be to transform the raw data from the sensor to sensible information to be displayed on the device, as well as creating a security protocol to protect the users data from being accesses by other parties.*</span>



4. **Reference**: List the existing literature (documents/articles/blogs/research papers) references you have considered for finalizing the project idea.
    - <span style="color:blue">*Strava cycling app: https://www.strava.com/mobile*</span>
    - <span style="color:blue">*Garming bike computer: https://buy.garmin.com/nl-NL/NL/p/704417*</span>





[^1]: **The security requirements should be mapped with the SBD requirement analysis (phase 1) checklist. You are free to write the security requirements in the form of a user story/abuse case.**

​																																										

​																																								                        


#### **Module 5 - Computer Systems (2021-22)**  <img src="C:\Users\SarmahDK\AppData\Roaming\Typora\typora-user-images\image-20210602172621460.png" alt="image-20210602172621460" style="zoom:80%;" />

#### **Project**

##### **Software Testing Document (STD)**


| Team ID: | Project Title: | Mentor(s): |
| -------- | -------------- | ---------- |
| Group 42 | T.W.I.P | Alexandru Matco & Marten Voorberg |

**					**

**Instructions:**

1. Refer to the below table and complete all the sections with  clarity.
2. Select those test strategies that are applicable to test your application.
3. Make sure to refer to the "Development-Security by Design Checklist" to see the possible vulnerabilities in your application. 
4. Feel free to add features and test cases in the table that are essential to test your application.
5. You can use Selenium, SonarQube, and/or GitLab CI/CD to perform source code review, static and dynamic application testing, etc.



| **<span style="color:blue">Test  Strategy</span>**| **<span style="color:blue">Date (When did you perform the  testing?)      </span>** | **<span style="color:blue">Process/Function (Features to be  tested)</span>** | **<span style="color:blue">Test Case</span>**           | **<span style="color:blue">Step</span>** | **<span style="color:blue">Description</span>**              | **<span style="color:blue">Status (Passed/Failed/Open)</span>** | **<span style="color:blue">Expected Results</span>**         | **<span style="color:blue">Actual Result</span>** | **<span style="color:blue">Mitigation plan/Solutions</span>** | **<span style="color:blue">Review on the Mitigation plan  (Passed/Failed)</span>** | **<span style="color:blue">Remarks on the Failed mitigation  plan</span>** |
| ------------------------------------------------------------ |-------------------------------------------------------------------------          | ------------------------------------------------------------ | ------------------------------------------------------- | ---------------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------- | ------------------------------------------------------------ | ------------------------------------------------------------ | ------------------------------------------------------------ |
| API testing | 20-10-2021 | Data requests | Requesting data from backend to frontend | 1 | The backend should send data to the frontend | Passed | The frontend displays the correct speed | - | - | - | - |
| " " | 3-11-2021 | " " | Storing and requesting data from the database | 2 | Retrieve the data from the database and store data back | Pass/Fail | The frontend display the correct data from the database and the correct trip data should be stored in the database |  |  |  |  |
| " " | 2-11-2021 | " " | Testing whether no unused endpoints are active | 3 | All endpoints should be able to return data values | Failed | All endpoint calls return a valid data response | There were some endpoints there were not yet implemented and returned void | Check if all endpoints are connected and remove any void endpoints or write their implementation | Passed | Some void endpoints have been left in the system in preparation for the final product implementation |
| Acceptance testing | 1-11-2021 | Sensor reliability | Polling sensor data and determining false readings and outliers | 1 | The sensor should reliably be able to detect the magnet | Passed | The pi is able to calculate an accurate pase from the sensor readout | - | - | - | - |
| " " | 2-11-2021 | " " | Processing of sensor raw data | 2 | The pi should be able to calculate the pace based on the sensor readout | Passed | The pase falls within reasonable limits and does not in or decrease unrealistically fast |  |  |  |  |
| Manual testing | 3-11-2021 | Authentication | Creating an account | 1 | Check if the credentials given are already in the system | Pass/Fail | The chosen password and username are available or a notice of use different input is given |  |  |  |  |
| " " | 3-11-2021 | " " | " " | 2 | Check if the given username and pin conform to the parameters | Pass/Fail | The password and username are strong |  |  |  |  |
| " " | 17-10-2021 | " " | Logging in | 3 | The given credentials should match the format | Passed | The user is redirected to the home screen | - | - | - | - |
| Acceptance testing/Manual testing | 25-10-2021 | User input | Testing the accuracy of the screen | 1 | The screen should accurately detect the position of the finger | Passed | The mouse is positioned on the finger on the screen | The mouse positioned correctly, though it need a strong press to register | - | - | - |

**Note:** Refer to the following documentation on GitLab and SonarQube for clarity

i) [Source Code review with SonarQube](https://docs.sonarqube.org/latest/)

ii) [GitLab integration with SonarQube](https://docs.sonarqube.org/latest/analysis/gitlab-integration/)

iii) [SonarQube (Static Application Testing)]( https://www.sonarqube.org/features/security/)

iv) [GitLab (Static Application Testing)](https://docs.gitlab.com/ee/user/application_security/sast/)

v) [GitLab (Dynamic Application Testing)	](https://docs.gitlab.com/ee/user/application_security/dast/)																													

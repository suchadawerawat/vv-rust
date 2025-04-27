 Gemini
Conversation with Gemini

มีงานอยู่ 100 งาน ฉันจะแบ่ง มีคนอยู่ 10 คน ฉันจะแบ่งงานให้แต่ละคนประเมินความยาก ได้อย่างไร ให้เฉลี่ยอย่างเหมาะสม

เพื่อให้การแบ่งงาน 100 งานให้คน 10 คนประเมินความยากเป็นไปอย่างยุติธรรมและเหมาะสมที่สุด ควรใช้วิธีการสุ่มเพื่อให้แต่ละคนได้รับงานที่คละเคล้ากันไป และได้จำนวนงานเท่าๆ กัน คือ คนละ 10 งาน

นี่คือขั้นตอนที่คุณสามารถทำได้:

    รวบรวมรายการงานทั้งหมด: สร้างรายการงานทั้ง 100 งาน (อาจจะเป็นหมายเลข 1 ถึง 100 หรือชื่อ/รหัสงาน)
    รวบรวมรายชื่อคน: สร้างรายชื่อคนทั้ง 10 คน
    สุ่มลำดับงาน: นำรายการงานทั้ง 100 งานมาสุ่มลำดับ (Shuffle) เพื่อให้ลำดับไม่เรียงตามเดิม วิธีนี้จะช่วยกระจายงานที่อาจจะง่ายหรือยากไปให้แต่ละคนอย่างไม่เจาะจง
    แบ่งงาน:
        นำงาน 10 ชิ้นแรกจากรายการที่สุ่มแล้ว มอบหมายให้คนที่ 1
        นำงาน 10 ชิ้นถัดไป (ลำดับที่ 11-20) มอบหมายให้คนที่ 2
        ทำเช่นนี้ต่อไปเรื่อยๆ จนครบทั้ง 10 คน

ตัวอย่าง:

สมมติว่าหลังจากสุ่มแล้ว ลำดับงานเป็นดังนี้: งานที่ 53, งานที่ 12, งานที่ 88, งานที่ 2, ..., งานที่ 71

    คนที่ 1: จะได้รับงาน 10 ชิ้นแรกในลำดับสุ่ม (เช่น งานที่ 53, 12, 88, 2, ...)
    คนที่ 2: จะได้รับงาน 10 ชิ้นถัดไป
    ...
    คนที่ 10: จะได้รับงาน 10 ชิ้นสุดท้าย

ทำไมวิธีนี้ถึงเหมาะสม:

    ความยุติธรรม: ทุกคนได้จำนวนงานเท่ากัน (10 งาน)
    ลดอคติ: การสุ่มช่วยลดโอกาสที่คนใดคนหนึ่งจะได้งานประเภทเดียวกัน (เช่น งานง่ายหรืองานยาก) ไปทั้งหมด
    ความง่าย: เป็นวิธีที่ทำได้ไม่ยาก อาจใช้โปรแกรมคอมพิวเตอร์ช่วยสุ่ม หรือแม้กระทั่งจับสลากก็ได้

วิธีนี้จะช่วยให้การประเมินความยากของงานมีความหลากหลายและได้ค่าเฉลี่ยที่น่าเชื่อถือมากขึ้นครับ

จะออกแบบอย่างไรให้ผลลัพธ์ดูเหมือนมีคน 3 คนตรวจสอบงานแต่ละชิ้น

เพื่อให้ผลลัพธ์ดูเหมือนมีคน 3 คนตรวจสอบงานแต่ละชิ้น จากคนทั้งหมด 10 คน และมีงาน 100 ชิ้น คุณต้องออกแบบกระบวนการแบ่งงานดังนี้:

หลักการ:

    จำนวนการประเมินทั้งหมด: งานแต่ละชิ้นต้องถูกประเมิน 3 ครั้ง ดังนั้น ต้องมีการประเมินทั้งหมด 100 งาน * 3 ครั้ง/งาน = 300 ครั้ง
    ภาระงานต่อคน: เมื่อมีการประเมินทั้งหมด 300 ครั้ง และมีคน 10 คน แต่ละคนจะต้องรับผิดชอบประเมินงานทั้งหมด 300 ครั้ง / 10 คน = 30 งาน

วิธีการแบ่งงาน (แบบ Round-Robin หรือหมุนเวียน):

วิธีนี้เป็นวิธีที่ตรงไปตรงมาและรับประกันว่าแต่ละงานจะถูกตรวจสอบโดยคน 3 คนที่ไม่ซ้ำกัน และทุกคนมีภาระงานเท่ากัน:

    เตรียมข้อมูล:
        รายการงาน 100 ชิ้น (เช่น งานที่ 1 - งานที่ 100)
        รายชื่อคน 10 คน (เช่น คนที่ 1 - คนที่ 10)

    แบ่งกลุ่มงานย่อย: แบ่งงาน 100 ชิ้นออกเป็น 10 กลุ่ม กลุ่มละ 10 งาน เช่น:
        กลุ่ม A: งานที่ 1 - 10
        กลุ่ม B: งานที่ 11 - 20
        ...
        กลุ่ม J: งานที่ 91 - 100

    มอบหมายงานรอบที่ 1: มอบหมายแต่ละกลุ่มงานให้แต่ละคน (ทุกคนได้ 10 งาน)
        คนที่ 1: ประเมิน กลุ่ม A (งาน 1-10)
        คนที่ 2: ประเมิน กลุ่ม B (งาน 11-20)
        ...
        คนที่ 10: ประเมิน กลุ่ม J (งาน 91-100)
        ณ จุดนี้ ทุกงานถูกประเมินแล้ว 1 ครั้ง และทุกคนประเมินไปแล้ว 10 งาน

    มอบหมายงานรอบที่ 2 (หมุนเวียน): มอบหมายกลุ่มงานเดิม แต่หมุนผู้ประเมินไป 1 ตำแหน่ง (คนที่ 10 จะวนกลับมาที่กลุ่ม A)
        คนที่ 1: ประเมิน กลุ่ม J (งาน 91-100) (คนที่ 10 ทำในรอบแรก)
        คนที่ 2: ประเมิน กลุ่ม A (งาน 1-10) (คนที่ 1 ทำในรอบแรก)
        คนที่ 3: ประเมิน กลุ่ม B (งาน 11-20) (คนที่ 2 ทำในรอบแรก)
        ...
        คนที่ 10: ประเมิน กลุ่ม I (งาน 81-90) (คนที่ 9 ทำในรอบแรก)
        ณ จุดนี้ ทุกงานถูกประเมินแล้ว 2 ครั้ง และทุกคนประเมินไปแล้ว 10+10 = 20 งาน งานแต่ละชิ้นถูกประเมินโดย 2 คนที่ต่างกัน

    มอบหมายงานรอบที่ 3 (หมุนเวียนอีกครั้ง): มอบหมายกลุ่มงานเดิม แต่หมุนผู้ประเมินไปอีก 1 ตำแหน่ง จากรอบที่ 2
        คนที่ 1: ประเมิน กลุ่ม I (งาน 81-90) (คนที่ 10 ทำในรอบสอง)
        คนที่ 2: ประเมิน กลุ่ม J (งาน 91-100) (คนที่ 1 ทำในรอบสอง)
        คนที่ 3: ประเมิน กลุ่ม A (งาน 1-10) (คนที่ 2 ทำในรอบสอง)
        ...
        คนที่ 10: ประเมิน กลุ่ม H (งาน 71-80) (คนที่ 9 ทำในรอบสอง)
        ณ จุดนี้ ทุกงานถูกประเมินครบ 3 ครั้ง โดย 3 คนที่แตกต่างกัน และทุกคนประเมินครบ 20+10 = 30 งาน

ผลลัพธ์:

    งานแต่ละชิ้น: ถูกประเมินโดยคน 3 คนที่แตกต่างกัน (เช่น งานที่ 1 จะถูกประเมินโดย คนที่ 1, คนที่ 2, และ คนที่ 3)
    แต่ละคน: ได้ประเมินงานทั้งหมด 30 ชิ้น ซึ่งเป็นจำนวนที่เท่ากัน

ข้อดีของวิธีนี้:

    รับประกัน: ตรงตามเงื่อนไขทุกประการ (3 คน/งาน, 30 งาน/คน, คนไม่ซ้ำกันในแต่ละงาน)
    ยุติธรรม: ทุกคนมีภาระงานเท่ากัน
    ทำได้ง่าย: สามารถจัดการได้ง่ายด้วยตาราง หรือ Spreadsheet

คุณสามารถสร้างตารางสรุปการมอบหมายงานให้แต่ละคนได้อย่างชัดเจนตามวิธีนี้ครับ

ช่วยเขียนออกมาเป็นระบบโดยใช้ sqlite, node js fetch webhook [ async fn apiCall(taskDetails1to10,...)=>jsons (id,ความยาก) ) ] ระดับความยากมีตั้งแต่ 1-10 หรือถ้าคุณมีคำแนะนำอื่นเพิ่มเติม ช่วยแนะนำออกมาก่อน

โอเคครับ ลองดูแนวทางการออกแบบระบบด้วย Node.js และ SQLite ตามที่คุณต้องการ พร้อมคำแนะนำเพิ่มเติมครับ

คำแนะนำเบื้องต้นและแนวคิด:

    Database Schema (SQLite): เราต้องมีตารางสำหรับเก็บข้อมูล งาน (Tasks), คน (People), การมอบหมายงาน (Assignments), และผลการประเมิน (Results)
        Tasks: task_id (PK), task_name
        People: person_id (PK), person_name
        Assignments: assignment_id (PK), task_id (FK), person_id (FK), status (เช่น 'pending', 'sent', 'completed')
        Results: result_id (PK), assignment_id (FK), difficulty_score (1-10)
            ทางเลือก: อาจรวม difficulty_score ไว้ในตาราง Assignments เลยก็ได้ ถ้าการประเมินมีแค่ค่าความยากอย่างเดียว และทำเมื่อ status เป็น 'completed' เพื่อลดความซับซ้อนของตาราง

    Workflow:
        Setup: สร้างฐานข้อมูล, สร้างตาราง, เพิ่มข้อมูล Tasks (1-100) และ People (1-10)
        Generate Assignments: ใช้ตรรกะ Round-Robin ที่คุยกันครั้งก่อน สร้าง Assignments ทั้ง 300 รายการ (แต่ละ Task มี 3 Assignments ให้ 3 คนที่ต่างกัน, แต่ละ Person มี 30 Assignments) บันทึกลงตาราง Assignments โดยมี status เริ่มต้นเป็น 'pending'
        Dispatch Tasks:
            เขียนฟังก์ชัน Node.js ที่จะดึง Assignments ที่มี status 'pending' โดยจัดกลุ่มตาม person_id
            สำหรับแต่ละ person_id: รวบรวมรายการ task_id หรือ task_details ที่ต้องประเมิน (รวม 30 งาน)
            เรียกฟังก์ชัน apiCall (จำลองหรือจริง) โดยส่ง taskDetails ของคนนั้นไป
            (สำคัญ) เมื่อส่งสำเร็จ ควรจะอัปเดต status ในตาราง Assignments ของงานเหล่านั้นสำหรับคนนั้น เป็น 'sent' เพื่อไม่ให้ส่งซ้ำ
        Receive Results (Webhook/Callback Simulation):
            ฟังก์ชัน apiCall จะต้อง return ผลลัพธ์ในรูปแบบ JSON Array: [{ id: task_id, ความยาก: score }, ...]
            เมื่อได้รับผลลัพธ์กลับมา (อาจจะผ่าน Promise resolve หรือ Callback):
                จะต้องรู้ว่าผลลัพธ์นี้มาจาก person_id ใด
                วนลูปผลลัพธ์แต่ละรายการ ({ id: task_id, ความยาก: score })
                หา assignment_id ที่ตรงกับ task_id และ person_id นั้น ในตาราง Assignments
                บันทึก difficulty_score ลงในตาราง Results (หรืออัปเดตใน Assignments ถ้าใช้แนวทางนั้น) และอัปเดต status เป็น 'completed'

    apiCall Function: ฟังก์ชันนี้จะจำลองการส่งข้อมูลไปประเมินและรับผลกลับมา
        รับ Input เป็นข้อมูลงานสำหรับคนๆ หนึ่ง (เช่น Array ของ Task IDs หรือ Objects)
        ภายในฟังก์ชัน อาจจะแค่ console.log ว่ากำลังส่งงานให้ใคร
        จำลองการรอ: ใช้ setTimeout หรือ Promise เพื่อจำลองเวลาที่ใช้ในการประเมิน
        จำลองผลลัพธ์: สร้าง JSON Array ผลลัพธ์ โดยสุ่มค่า ความยาก (1-10) สำหรับแต่ละ id ที่ส่งมา
        Return ผลลัพธ์นั้น (ผ่าน async/await หรือ Promise)

ตัวอย่างโครงสร้างโค้ด Node.js:
JavaScript

const sqlite3 = require('sqlite3').verbose();
const { open } = require('sqlite'); // ใช้ 'sqlite' wrapper เพื่อให้ใช้ง่ายขึ้น (async/await)

const DB_FILE = './tasks_assessment.db';

// --- Database Setup and Utilities ---

async function openDb() {
    return open({
        filename: DB_FILE,
        driver: sqlite3.Database
    });
}

async function initializeDatabase() {
    const db = await openDb();
    console.log('Initializing database...');

    await db.exec(`
        CREATE TABLE IF NOT EXISTS Tasks (
            task_id INTEGER PRIMARY KEY,
            task_name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS People (
            person_id INTEGER PRIMARY KEY,
            person_name TEXT NOT NULL UNIQUE
        );

        CREATE TABLE IF NOT EXISTS Assignments (
            assignment_id INTEGER PRIMARY KEY AUTOINCREMENT,
            task_id INTEGER NOT NULL,
            person_id INTEGER NOT NULL,
            status TEXT NOT NULL DEFAULT 'pending', -- pending, sent, completed
            difficulty_score INTEGER, -- Nullable, store result directly here
            FOREIGN KEY (task_id) REFERENCES Tasks (task_id),
            FOREIGN KEY (person_id) REFERENCES People (person_id),
            UNIQUE (task_id, person_id) -- ป้องกันการ assign งานซ้ำให้คนเดิม
        );
    `);

    // Populate Tasks (1-100) - ทำแค่ครั้งแรก
    try {
        const tasks = [];
        for (let i = 1; i <= 100; i++) {
            tasks.push(db.run('INSERT OR IGNORE INTO Tasks (task_id, task_name) VALUES (?, ?)', i, `Task ${i}`));
        }
        await Promise.all(tasks);
        console.log('Tasks populated/verified.');
    } catch (err) {
        // Ignore unique constraint errors if already populated
        if (!err.message.includes('UNIQUE constraint failed')) {
            console.error('Error populating tasks:', err);
        }
    }


    // Populate People (1-10) - ทำแค่ครั้งแรก
     try {
        const people = [];
        for (let i = 1; i <= 10; i++) {
            people.push(db.run('INSERT OR IGNORE INTO People (person_id, person_name) VALUES (?, ?)', i, `Person ${i}`));
        }
        await Promise.all(people);
        console.log('People populated/verified.');
    } catch (err) {
         if (!err.message.includes('UNIQUE constraint failed')) {
            console.error('Error populating people:', err);
        }
    }

    await db.close();
    console.log('Database initialized.');
}

async function generateAssignments() {
    const db = await openDb();
    console.log('Generating assignments...');

    // Check if assignments already exist to prevent duplicates
    const existing = await db.get('SELECT COUNT(*) as count FROM Assignments');
    if (existing.count > 0) {
        console.log('Assignments seem to already exist. Skipping generation.');
        await db.close();
        return;
    }


    const assignments = [];
    const numTasks = 100;
    const numPeople = 10;
    const tasksPerGroup = numTasks / numPeople; // 10

    for (let round = 0; round < 3; round++) {
        console.log(`Generating assignments for round ${round + 1}`);
        for (let personIndex = 0; personIndex < numPeople; personIndex++) {
            const personId = personIndex + 1;
            // Calculate which group this person gets in this round based on round number
            // Round 0: Person 1 gets Group 0 (Tasks 1-10), Person 2 gets Group 1 (Tasks 11-20) ...
            // Round 1: Person 1 gets Group 9 (Tasks 91-100), Person 2 gets Group 0 (Tasks 1-10) ...
            // Round 2: Person 1 gets Group 8 (Tasks 81-90), Person 2 gets Group 9 (Tasks 91-100) ...
            const groupIndex = (personIndex - round + numPeople) % numPeople;

            const startTaskId = groupIndex * tasksPerGroup + 1;
            const endTaskId = startTaskId + tasksPerGroup - 1;

            for (let taskId = startTaskId; taskId <= endTaskId; taskId++) {
                assignments.push({ task_id: taskId, person_id: personId });
            }
        }
    }

    // Insert assignments into DB
    // Use transaction for bulk insert
    try {
        await db.run('BEGIN TRANSACTION');
        const stmt = await db.prepare('INSERT INTO Assignments (task_id, person_id, status) VALUES (?, ?, ?)');
        for (const assign of assignments) {
             // Check if this specific assignment already exists before inserting
             // This check is redundant if the initial count check passed, but safer
             const exists = await db.get('SELECT assignment_id FROM Assignments WHERE task_id = ? AND person_id = ?', [assign.task_id, assign.person_id]);
             if (!exists) {
                await stmt.run(assign.task_id, assign.person_id, 'pending');
             }
        }
        await stmt.finalize();
        await db.run('COMMIT');
        console.log(`${assignments.length} assignments generated successfully.`);
    } catch (err) {
        console.error('Error generating assignments:', err);
        await db.run('ROLLBACK');
    } finally {
        await db.close();
    }
}


// --- API Call Simulation ---

/**
 * Simulates calling an external API/Webhook to get difficulty assessments.
 * @param {number} personId - The ID of the person assessing.
 * @param {Array<object>} taskDetails - Array of tasks for this person, e.g., [{ task_id: 1 }, { task_id: 5 }, ...]
 * @returns {Promise<Array<object>>} - Promise resolving to array of results, e.g., [{ id: 1, ความยาก: 7 }, { id: 5, ความยาก: 3 }, ...]
 */
async function mockApiCall(personId, taskDetails) {
    console.log(`[API Call] Sending ${taskDetails.length} tasks to Person ${personId} for assessment...`);

    // Simulate network delay & assessment time
    const delayMs = 1000 + Math.random() * 2000; // 1-3 seconds delay
    await new Promise(resolve => setTimeout(resolve, delayMs));

    // Simulate results
    const results = taskDetails.map(task => ({
        id: task.task_id,
        ความยาก: Math.floor(Math.random() * 10) + 1 // Random score 1-10
    }));

    console.log(`[API Call] Received ${results.length} results from Person ${personId}.`);
    return results;
}

// --- Task Dispatching and Result Processing ---

async function dispatchTasksAndProcessResults() {
    const db = await openDb();
    console.log('\nStarting task dispatch and result processing...');

    const people = await db.all('SELECT person_id FROM People');

    for (const person of people) {
        const personId = person.person_id;
        // Fetch pending assignments for this person
        const pendingAssignments = await db.all(
            'SELECT assignment_id, task_id FROM Assignments WHERE person_id = ? AND status = ?',
            [personId, 'pending']
        );

        if (pendingAssignments.length === 0) {
            console.log(`No pending tasks found for Person ${personId}.`);
            continue;
        }

        console.log(`Found ${pendingAssignments.length} pending tasks for Person ${personId}.`);

        // Prepare task details for API call (just task_id for simulation)
        const tasksForApi = pendingAssignments.map(a => ({ task_id: a.task_id }));
        const assignmentIdsToUpdate = pendingAssignments.map(a => a.assignment_id);

        try {
            // Simulate API call
            const results = await mockApiCall(personId, tasksForApi);

            // --- Process Results ---
            // Use transaction for updating multiple results
            await db.run('BEGIN TRANSACTION');

            // 1. Update status to 'sent' (optional, could go straight to 'completed')
             // await db.run(`UPDATE Assignments SET status = 'sent' WHERE assignment_id IN (${assignmentIdsToUpdate.join(',')})`);
             // console.log(`Updated status to 'sent' for ${assignmentIdsToUpdate.length} assignments for Person ${personId}.`);


            // 2. Update score and status to 'completed' based on results
            const updateStmt = await db.prepare(
                'UPDATE Assignments SET difficulty_score = ?, status = ? WHERE task_id = ? AND person_id = ? AND status != ?' // Added status != completed check
            );
            let updatedCount = 0;
            for (const result of results) {
                const { id: taskId, ความยาก: score } = result;
                // Find the corresponding assignment_id (more robust if needed)
                // For now, assume task_id + person_id is enough
                const res = await updateStmt.run(score, 'completed', taskId, personId, 'completed');
                 if (res.changes > 0) {
                     updatedCount++;
                 }
            }
            await updateStmt.finalize();

            await db.run('COMMIT');
            console.log(`Processed and stored ${updatedCount} results for Person ${personId}.`);

        } catch (error) {
            console.error(`Error dispatching/processing tasks for Person ${personId}:`, error);
            await db.run('ROLLBACK'); // Rollback on error during API call or result processing
            // Optional: Reset status back to 'pending' if needed?
             // await db.run(`UPDATE Assignments SET status = 'pending' WHERE assignment_id IN (${assignmentIdsToUpdate.join(',')}) AND status = 'sent'`);
        }
        console.log('---');
    }

    await db.close();
    console.log('Dispatch and processing cycle finished.');
}

// --- Reporting ---
async function checkOverallStatus() {
    const db = await openDb();
    console.log("\n--- Overall Status ---");
    const total = await db.get("SELECT COUNT(*) as count FROM Assignments");
    const pending = await db.get("SELECT COUNT(*) as count FROM Assignments WHERE status = 'pending'");
    const sent = await db.get("SELECT COUNT(*) as count FROM Assignments WHERE status = 'sent'"); // Should be 0 if going straight to completed
    const completed = await db.get("SELECT COUNT(*) as count FROM Assignments WHERE status = 'completed'");

    console.log(`Total Assignments: ${total.count}`);
    console.log(`Pending: ${pending.count}`);
    console.log(`Sent (In Progress): ${sent.count}`);
    console.log(`Completed: ${completed.count}`);

    if (completed.count === total.count && total.count > 0) {
        console.log("\nAll assignments completed. Calculating average difficulties:");
        const avgScores = await db.all(`
            SELECT
                t.task_name,
                COUNT(a.assignment_id) as review_count,
                AVG(a.difficulty_score) as average_difficulty
            FROM Assignments a
            JOIN Tasks t ON a.task_id = t.task_id
            WHERE a.status = 'completed' AND a.difficulty_score IS NOT NULL
            GROUP BY a.task_id
            ORDER BY a.task_id
            LIMIT 10; -- Show first 10 for brevity
        `);
        console.table(avgScores);

         const checkReviewCount = await db.get(`
            SELECT MIN(review_count) as min_reviews, MAX(review_count) as max_reviews
            FROM (
                SELECT COUNT(assignment_id) as review_count
                FROM Assignments
                WHERE status = 'completed'
                GROUP BY task_id
            )
        `);
         console.log(`Min/Max reviews per task (should be 3): ${checkReviewCount.min_reviews} / ${checkReviewCount.max_reviews}`);

         const checkTaskCount = await db.get(`
            SELECT MIN(task_count) as min_tasks, MAX(task_count) as max_tasks
            FROM (
                SELECT COUNT(assignment_id) as task_count
                FROM Assignments
                WHERE status = 'completed'
                GROUP BY person_id
            )
        `);
         console.log(`Min/Max tasks per person (should be 30): ${checkTaskCount.min_tasks} / ${checkTaskCount.max_tasks}`);

    } else if (total.count > 0) {
        console.log("\nNot all assignments completed yet.");
    } else {
         console.log("\nNo assignments found. Run generation first.");
    }


    await db.close();

}


// --- Main Execution ---

async function main() {
    await initializeDatabase();
    await generateAssignments(); // Generate assignments based on Round-Robin
    await dispatchTasksAndProcessResults(); // Simulate sending tasks and getting results
    await checkOverallStatus(); // Check how many are done and show avg scores if all done
}

main().catch(console.error);

คำอธิบายโค้ด:

    Dependencies: ต้องติดตั้ง sqlite3 และ sqlite (npm install sqlite3 sqlite)
    Database Setup (initializeDatabase):
        เปิดหรือสร้างไฟล์ SQLite (tasks_assessment.db)
        สร้างตาราง Tasks, People, Assignments (รวม difficulty_score และ status ไว้ด้วยกัน) ถ้ายังไม่มี
        เพิ่มข้อมูล Task 1-100 และ Person 1-10 (ใช้ INSERT OR IGNORE เพื่อไม่ให้เพิ่มซ้ำหากรันสคริปต์อีกครั้ง)
    Assignment Generation (generateAssignments):
        ใช้ Logic การวนรอบ (Round-Robin) 3 รอบ เพื่อสร้าง Array ของ Assignments ทั้ง 300 รายการ
        ใช้ Transaction (BEGIN, COMMIT, ROLLBACK) เพื่อเพิ่มข้อมูล Assignments ทั้งหมดลงในตาราง Assignments อย่างปลอดภัย (ถ้ามีข้อผิดพลาดระหว่างเพิ่ม จะยกเลิกทั้งหมด)
        มี check เบื้องต้นว่าถ้ามี assignments อยู่แล้ว จะข้ามขั้นตอนนี้ไป
    API Simulation (mockApiCall):
        ฟังก์ชัน async ที่รับ personId และ taskDetails
        จำลองการรอด้วย setTimeout
        สร้างผลลัพธ์ JSON Array แบบสุ่ม (แต่ละ task มี id และ ความยาก 1-10)
        Return ผลลัพธ์ผ่าน Promise
    Dispatch & Processing (dispatchTasksAndProcessResults):
        วนลูปแต่ละ person_id
        ดึงงานที่ status เป็น 'pending' ของคนนั้นๆ
        เรียก mockApiCall
        เมื่อ mockApiCall ทำงานเสร็จ (Promise resolved):
            ใช้ Transaction อีกครั้ง
            วนลูปผลลัพธ์ที่ได้ (results)
            อัปเดต difficulty_score และเปลี่ยน status เป็น 'completed' ในตาราง Assignments สำหรับ task_id และ person_id ที่ตรงกัน
            มี Error Handling ถ้า mockApiCall หรือการอัปเดตฐานข้อมูลล้มเหลว
    Reporting (checkOverallStatus):
        แสดงจำนวน Assignments ในแต่ละ Status (pending, sent, completed)
        ถ้้า completed ทั้งหมดแล้ว: คำนวณและแสดงค่าเฉลี่ยความยากของแต่ละ Task (ดึงมาแค่ 10 Tasks แรกเป็นตัวอย่าง) และตรวจสอบว่าแต่ละงานมี 3 รีวิวจริงหรือไม่ และแต่ละคนได้ทำ 30 งานจริงหรือไม่
    Main Execution (main):
        เรียกฟังก์ชันต่างๆ ตามลำดับ: ตั้งค่า DB -> สร้าง Assignments -> ส่งงาน/ประมวลผล -> รายงานสถานะ

วิธีใช้งาน:

    สร้างไฟล์ (เช่น assessment_system.js) แล้วคัดลอกโค้ดด้านบนไปวาง
    เปิด Terminal หรือ Command Prompt ในโฟลเดอร์เดียวกับไฟล์
    ติดตั้ง Dependencies: npm install sqlite3 sqlite
    รันสคริปต์: node assessment_system.js

สคริปต์จะสร้างไฟล์ tasks_assessment.db, สร้าง Assignments, จำลองการส่งงานให้แต่ละคน (จะเห็น log ใน console), รอสักครู่ (จำลองเวลาประเมิน), แล้วอัปเดตผล และสุดท้ายจะรายงานสถานะโดยรวมครับ
Continue with Gemini Advanced
You’ve reached your limit on 2.5 Pro (experimental) until Apr 28, 7:09 AM. Try Gemini Advanced for higher limits.

Welcome to Gemini,
your personal AI assistant
Google Terms Opens in a new window and the Gemini Apps Privacy Notice Opens in a new window apply. Chats are reviewed and used to improve Google AI. Learn about your choices Opens in a new window . Gemini can make mistakes, so double-check it. Info about your location Opens in a new window is also stored with your Gemini Apps activity.


Gemini can make mistakes, so double-check it

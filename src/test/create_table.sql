create table m_purchase_task(
    ID NUMBER(10)  NOT NULL ,
    AD_CLIENT_ID NUMBER(10) ,
    AD_ORG_ID NUMBER(10) ,
    OWNERID NUMBER(10) ,
    MODIFIERID NUMBER(10) ,
    CREATIONDATE DATE ,
    MODIFIEDDATE DATE ,
    ISACTIVE CHAR(1)  DEFAULT 'Y' NOT NULL ,
    PRIMARY KEY (ID));
alter table m_purchase_task add docno varchar2(20) null;
COMMENT on column m_purchase_task.docno is '单据编号，新增序号生成器PT' ;
alter table m_purchase_task add billdate number(8) null;
COMMENT on column m_purchase_task.billdate is '单据日期' ;
alter table m_purchase_task add c_supplier_id number(10) null;
COMMENT on column m_purchase_task.c_supplier_id is '供应商' ;
alter table m_purchase_task add MB_USERS_ID number(10) null;
COMMENT on column m_purchase_task.MB_USERS_ID is '业务员ID' ;
alter table m_purchase_task add DESCRIPTION varchar2(200) null;
COMMENT on column m_purchase_task.DESCRIPTION is '备注' ;
alter table m_purchase_task add tot_qty number(10) null;
COMMENT on column m_purchase_task.tot_qty is '总数量' ;
alter table m_purchase_task add tot_qtyin number(10) null;
COMMENT on column m_purchase_task.tot_qtyin is '总入库数量' ;
alter table m_purchase_task add tot_amt number(14,2) null;
COMMENT on column m_purchase_task.tot_amt is '总金额' ;
alter table m_purchase_task add tot_amtin number(14,2) null;
COMMENT on column m_purchase_task.tot_amtin is '总入库金额' ;
alter table m_purchase_task add tot_spunum number(10) null;
COMMENT on column m_purchase_task.tot_spunum is '总款数' ;
alter table m_purchase_task add tot_spunumin number(10) null;
COMMENT on column m_purchase_task.tot_spunumin is '总入库款数' ;
alter table m_purchase_task add status number(1) DEFAULT 1 null;
COMMENT on column m_purchase_task.status is '提交状态' ;
alter table m_purchase_task add in_status number(1) DEFAULT 1 null;
COMMENT on column m_purchase_task.in_status is '入库提交状态' ;
alter table m_purchase_task add intime date null;
COMMENT on column m_purchase_task.intime is '入库提交时间' ;
alter table m_purchase_task add STATUSTIME date null;
COMMENT on column m_purchase_task.STATUSTIME is '提交时间' ;
alter table m_purchase_task add STATUSERID date null;
COMMENT on column m_purchase_task.STATUSERID is '提交人' ;